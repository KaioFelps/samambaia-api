mod controllers;
mod repository;
mod service;

pub use controllers::generate_controller as controller;
pub use repository::generate_repository as repository;
pub use service::generate_service as service;
