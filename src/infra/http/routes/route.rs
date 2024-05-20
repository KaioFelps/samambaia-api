use actix_web::web::ServiceConfig;

pub trait RouteTrait {
    fn register(cfg: &mut ServiceConfig);
}
