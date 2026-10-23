use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Semaphore, broadcast};
use tokio::task::JoinSet;

use crate::job::{Job, JobHandler};
use crate::queue::JobQueue;

pub struct WorkerConfig {
    pub concurrency: usize,
    /// Interval to wait before trying to poll again (when the queue is empty).
    pub poll_interval: Duration,
    pub job_timeout: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            poll_interval: Duration::from_secs(1),
            job_timeout: Duration::from_secs(300),
        }
    }
}

/// Worker that processes jobs from the [`JobQueue`](super::queue::JobQueue)
pub struct Worker<T, H> {
    queue: JobQueue,
    handler: Arc<H>,
    config: WorkerConfig,
    shutdown_rx: broadcast::Receiver<()>,
    _marker: std::marker::PhantomData<T>,
}

impl<T, H> Worker<T, H>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Clone + Send + Sync + 'static,
    H: JobHandler<T> + 'static,
{
    pub fn new(
        queue: JobQueue,
        handler: H,
        config: WorkerConfig,
        shutdown_rx: broadcast::Receiver<()>,
    ) -> Self {
        Self {
            queue,
            handler: Arc::new(handler),
            config,
            shutdown_rx,
            _marker: std::marker::PhantomData,
        }
    }

    /// Waits `delay` blocking the current green-thread. (But wakes up
    /// if shutdown is requested.)
    ///
    /// Returns `true` when requested to shutdown.
    async fn wait_or_shutdown(&mut self, delay: Duration) -> bool {
        tokio::select! {
            _ = self.shutdown_rx.recv() => true,
            _ = tokio::time::sleep(delay) => false,
        }
    }

    /// Start the worker
    pub async fn run(mut self) {
        log::info!("Worker starting (concurrency: {})", self.config.concurrency);

        let semaphore = Arc::new(Semaphore::new(self.config.concurrency));
        let mut tasks: JoinSet<()> = JoinSet::new();

        loop {
            let permit = tokio::select! {
                _ = self.shutdown_rx.recv() => {
                    log::info!("Worker received shutdown signal");
                    break;
                }
                permit = semaphore.clone().acquire_owned() => match permit {
                    Ok(permit) => permit,
                    Err(_) => break,
                },
            };

            // cleans concluded tasks (non-blocking)
            while let Some(result) = tasks.try_join_next() {
                if let Err(e) = result {
                    log::error!("Job task failed (error: {e})");
                }
            }

            // sleep after polling since dequeing is non-blocking
            match self.queue.dequeue::<T>().await {
                Ok(Some(job)) => {
                    let queue = self.queue.clone();
                    let handler = self.handler.clone();
                    let timeout = self.config.job_timeout;

                    tasks.spawn(async move {
                        let _permit = permit;
                        process_job(queue, handler, job, timeout).await;
                    });
                }
                Ok(None) => {
                    drop(permit);
                    if self.wait_or_shutdown(self.config.poll_interval).await {
                        log::info!("Worker received shutdown signal");
                        break;
                    }
                }
                Err(e) => {
                    drop(permit);
                    log::error!("Error dequeuing job (error: {e:?})");
                    if self.wait_or_shutdown(Duration::from_secs(1)).await {
                        log::info!("Worker received shutdown signal");
                        break;
                    }
                }
            }
        }

        // wait for in-progress jobs to finish before stopping
        while let Some(result) = tasks.join_next().await {
            if let Err(e) = result {
                log::error!("Job task failed (error: {e})");
            }
        }
        log::info!("Worker stopped");
    }
}

/// Process a single job with timeout
async fn process_job<T, H>(queue: JobQueue, handler: Arc<H>, job: Job<T>, timeout: Duration)
where
    T: serde::Serialize + Clone + Send + Sync,
    H: JobHandler<T>,
{
    let job_id = job.meta.id;
    let job_type = job.meta.job_type.clone();

    log::info!(
        "Processing job\n(job_id: {job_id})\n(job_type: {job_type})\n(attempt: {})",
        job.meta.attempts + 1
    );

    let start = std::time::Instant::now();

    let result = tokio::time::timeout(timeout, handler.handle(&job)).await;

    let duration = start.elapsed();

    match result {
        Ok(Ok(())) => {
            if let Err(e) = queue.complete(job_id).await {
                log::error!("Failed to mark job complete\n(job_id: {job_id})\n(error: {e:?})");
            }

            log::info!(
                "Job completed successfully\n(job_id: {job_id})\n(duration_ms: {})",
                duration.as_millis()
            );
        }
        Ok(Err(e)) => {
            let message = e.to_string();
            log::warn!("Job failed\n(job_id: {job_id})\n(error: {message})");

            if let Err(qe) = queue.fail(job, &message).await {
                log::error!("Failed to mark job as failed\n(job_id: {job_id})\n(error: {qe:?})");
            }
        }
        Err(_) => {
            log::error!(
                "Job timed out\n(job_id: {job_id})\n(timeout_seconds: {})",
                timeout.as_secs()
            );

            if let Err(qe) = queue.fail(job, "Job timed out").await {
                log::error!("Failed to mark job as failed\n(job_id: {job_id})\n(error: {qe:?})");
            }
        }
    }
}
