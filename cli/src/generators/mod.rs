mod service;
mod repository;
mod controllers;

pub use service::generate_service as service;
pub use repository::generate_repository as repository;
pub use controllers::generate_controller as controller;