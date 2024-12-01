# Changelog
This file contains the most notable changes of each released version, starting from the one in which the changelog
has been introduced.

## v0.5.0
- Database has been improved by 100%;
    -   database connection are now created only once at the application bootstrap;
    -   repositories now holds a reference for the database connection, instead of an owned connection;
    -   factories now requires a reference to a `SeaService` (database connection) in order to instantiate the
        repositories, and thus the services;
    -   controllers can now retrieve a reference to the `SeaService` struct by using the `actix_web::web::Data`
        extractor; they must use it for executing the services factories.
