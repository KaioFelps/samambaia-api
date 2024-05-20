use actix_web::web::ServiceConfig;

/**
 # Controller Trait
 Trait that every controller struct should implement.
 
 It guarantees that the struct will contain the `register` and other controller-mandatory functions.
 */
pub trait ControllerTrait {
    /**
     # Register
     A function that should use the `cfg` parameter to register every route related to the controller.

     ### Example of usage
     ```rs
     // users_controller.rs
     pub fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/users")
            // CREATE
            .route("/new", web::post().to(Self::create))

            // UPDATE
            .route("/{id}/update", web::put().to(Self::update).wrap(SomeSampleMiddleware)
            //...
        );
    }
     ```
     */
    fn register(cfg: &mut ServiceConfig);
}