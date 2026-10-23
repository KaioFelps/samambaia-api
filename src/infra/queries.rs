//! Queries can use whatever they want to access datastores.
//! They are a big mess and are responsible for getting to the controller
//! a view of whatever data the controller want. E.g.: when the controller
//! wants an aggregate of data, a query can fetch the required fields directly
//! with no need to put up traits and repositories and services and other
//! layers, like more entities and aggregates, for something that is just
//! a readonly view and that is to be used by a single entrypoint in the
//! whole application.

pub mod audit_logs;
