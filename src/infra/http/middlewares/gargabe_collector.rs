use actix_web::dev::{self, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use actix_web_lab::__reexports::futures_util::future::LocalBoxFuture;
use rand::Rng;
use std::future::{ready, Ready};

use crate::configs::app::APP_CONFIG;
use crate::configs::file_sessions::clean_expired_sessions;

pub struct GarbageCollectorMiddleware;
impl<S, B> Transform<S, ServiceRequest> for GarbageCollectorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GarbageCollectorMiddlewareTransform<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(GarbageCollectorMiddlewareTransform { service }))
    }
}

pub struct GarbageCollectorMiddlewareTransform<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for GarbageCollectorMiddlewareTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let lottery_num = rand::thread_rng().gen_range(0..APP_CONFIG.lottery[1]);
        let is_raffled = lottery_num <= APP_CONFIG.lottery[0];

        let fut = self.service.call(request);

        Box::pin(async move {
            let res = fut.await?;

            if is_raffled {
                log::info!("Request raffled to run the garbage collector.");
                let _ = clean_expired_sessions().await;
            }

            Ok(res)
        })
    }
}
