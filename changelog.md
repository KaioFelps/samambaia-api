# Changelog
This file contains the most notable changes of each released version, starting from the one in which the changelog
has been introduced.

## Unreleased
- nothing.

## Samambaia@0.5.0
### Added
- introduced **Announcements** domain;
    -   DB model, entity and domain entity;
    -   CRUD services;
    -   Sea Repository;
    -   CRUD services factories;
    -   DTOs, presenter and controller;
- added CI automated tests + lint checks.

### Changed
- added `SeaMapper` trait that defines the 4 most common methods a SEA-ORM mapper might have;
- implemented `SeaMapper` for every *mapper* and fixed repositories broken method calls due to the *mappers* changes;
- made Sea repositories constructors synchronous;
- made services factories synchronous;
- move repositories dependencies out of `Box` in every service;
- group services by domain;
- group factories by domain;
- rename the system to **Samambaia**.

### Changed
- database performance has been improved by 92%:
    -   database connection are now created only once at the application bootstrap;
    -   repositories now holds a reference for the database connection, instead of an owned connection;
    -   factories now requires a reference to a `SeaService` (database connection) in order to instantiate the
        repositories, and thus the services;
    -   controllers can now retrieve a reference to the `SeaService` struct by using the `actix_web::web::Data`
        extractor; they must use it for executing the services factories.

## Hubbitos@0.4.0
### Added
- added a full authentication middleware (that does not require from_fn).

### Changed
- updated outdated dependencies and fixed code broken by their breaking changes;
- updated `Validator` crate and fixed new regex path format at `dtos/create-user` and `dtos/update-user.rs`;
- changed `from_fn` import from `actix_web_lab` to `actix_web` crate on every controller & authentication middleware.

## Hubbitos@0.3.1
### Fixed
- fixed `udpate` method from Free Badges controller.

## Hubbitos@0.3.0
### Added
- introduced **Free Badges**:
    - domain entity, database table + migration, ORM model;
    - CRUD services;
    - create and update DTOs;
    - added controller and routes and registered it to api routes.

## Hubbitos@0.2.0
### Added
- added **Article Tags** DTOs;
- added **Article Tags** Presenter;
- added **Article Tags** Controller.

## Hubbitos@0.1.0
### Added
- introduced **Users** domain + CRUD + migrations and entities;
- introduced **Team Roles** domain + CRUD + migrations and entities;
- introduced **Team Users** domain + CRUD + migrations and entities;
- introduced **Articles** domain + CRUD + migrations and entities;
- introduced **Article Tags** domain + CRUD + migrations and entities;
- introduced **Comments** domain + CRUD + migrations and entities;
- introduced **Comments Reports** domain + CRUD + migrations and entities.

### Changed
- started using releases for managing the fansite versioning.