mod service;
mod repository_flat;

pub use service::get_service_template as get_service_template;
pub use repository_flat::get_repository_flat_template as get_repository_flat_template;
pub mod scaffold_repository;