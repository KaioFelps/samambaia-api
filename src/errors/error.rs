use std::error::Error;

pub trait DomainErrorTrait: Error {
    fn code(&self) -> &u16;
    fn message(&self) -> &String;
}