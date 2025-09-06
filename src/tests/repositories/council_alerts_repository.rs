use std::error::Error;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::domain::domain_entities::council_alert::{CouncilAlert, CouncilAlertDraft};
use crate::domain::repositories::council_alerts_repository::CouncilAlertRepositoryTrait;
use crate::tests::repositories::LocalDb;

#[derive(Clone)]
pub struct InMemoryCouncilAlertRepository {
    pub council_alert_db: LocalDb<CouncilAlert>,
}

impl InMemoryCouncilAlertRepository {
    pub fn new() -> Self {
        Self::with_db(Arc::new(Mutex::new(Vec::new())))
    }

    pub fn with_db(council_alert_db: LocalDb<CouncilAlert>) -> Self {
        Self { council_alert_db }
    }
}

#[async_trait]
impl CouncilAlertRepositoryTrait for InMemoryCouncilAlertRepository {
    async fn create(
        &self,
        council_alert: CouncilAlertDraft,
    ) -> Result<CouncilAlert, Box<dyn Error>> {
        let mut db = self.council_alert_db.lock().unwrap();
        let council_alert = council_alert.into_council_alert(db.len() as i32 + 1);
        db.push(council_alert.clone());
        Ok(council_alert)
    }

    async fn find_all(&self) -> Result<Vec<CouncilAlert>, Box<dyn Error>> {
        let (mut pinned_alerts, mut alerts): (Vec<_>, Vec<_>) = self
            .council_alert_db
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .partition(|alert| alert.pinned());

        alerts.sort_by_key(|alert| std::cmp::Reverse(alert.created_at()));
        pinned_alerts.sort_by_key(|alert| std::cmp::Reverse(alert.created_at()));
        pinned_alerts.append(&mut alerts);

        Ok(pinned_alerts)
    }
}
