# Using Hubbitos CLI for generating boilerplates

Consider that the CLI expects few files to exist and some crates to be available on your application:
## Files
- `crate::error::DomainError`: an enum or an struct that will be the `Err` variant from the services returns.
- `super::controller::ControllerTrait`: a trait that must be available in a `controller.rs` module in the same directory as your controllers output directory.
- `crate::super::AppResponse`: a type alias to Result<HttpResponse, DomainError> that must be available in the `mod.rs` file from your controllers output directory.

## Crates
- `mockall`: It's used in the repositories stubs;
- `async-trait`: It's used in the repositories stubs;
- `actix-web`: It's used in the controllers stubs.

## Enabling the CLI
First of all, compile the cli with `cargo build --release -p hubbitos-cli`.

### Bash
Open your bash and type `source .bashrc` and you good to go with Hubbitos CLI.

### Powershell
Open your powershell terminal and type `. ./src/cli.ps1` and you good to go with Hubbitos CLI.

## Services
Generate a service file on `src/domain/services` and append it in `src/domain/services/mod.rs`.
```bash
hubbitos-cli generate service "service name lowercase splitten by spaces"
```

This will generate a service just like:
```rust
use crate::error::DomainError;

pub struct ServiceNameParams {}

pub struct ServiceNameService {}

impl ServiceNameService {
    pub fn new() -> Self {
        ServiceNameService {

        }
    }

    pub async fn exec(&self, params: ServiceNameParams) -> Result<(), DomainError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {}
```

## Repository
If for some reason you aim to generate only a repository with no entity, the command below will generate a new file on `src/domain/repositories` and append it to `src/domain/repositories/mod.rs`.
```bash
hubbitos-cli generate repository "repository name"
```

This will generate a repository like:
```rust
use async_trait::async_trait;
use std::error::Error;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RepositoryNameRepositoryTrait {}
```

## Controller
Generate a controller on `src/infra/http/controllers/` and append it on `src/infra/http/controller/mod.rs`.
```bash
hubbitos-cli generate controller "user"
# you can also overwrite the default directory with -o flag
# note that the domain entity is written in lowercase singular
# for compound names, simply add whitespaces, ie: "comment report"
```

This command would generate a following `src/infra/http/controllers/users_controller.rs` file:
```rust
use actix_web::{web, HttpResponse};
use super::controller::ControllerTrait;
use super::AppResponse;

pub struct UsersController;

impl ControllerTrait for UsersController {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/users")
            // CREATE
            .route("/new", web::post().to(Self::create))

            // READ
            .route("/{id}", web::get().to(Self::get))
            .route("/list", web::get().to(Self::list))
            
            // UPDATE
            .route("/{id}/update", web::put().to(Self::update))

            // DELETE
            .route("/delete", web::put().to(Self::delete))
        );
    }
}

impl UsersController {
    async fn create() -> AppResponse {
        // let service = service_factory::exec()?;
        // service.exec()?;

        Ok(HttpResponse::Created().finish())
    }

    async fn get() -> AppResponse {
        Ok(HttpResponse::Ok().finish())
    }

    async fn list() -> AppResponse {
        Ok(HttpResponse::Ok().finish())
    }

    async fn update() -> AppResponse {
        Ok(HttpResponse::NoContent().finish())
    }

    async fn delete() -> AppResponse {
        Ok(HttpResponse::NoContent().finish())
    }
}

```

## The output directory flag
This flag has 5 aliases:
- `-o`
- `--output`
- `--output-dir`
- `--dir`
- `-d`

All of them do the exactly same thing: modify the artifact output directory.
Each artifact (controller, repository, service) has a default directory (described in their respective topics).
To overwrite the target directory, you can pass one of the aliases of `-output` flag followed by the desired directory.

Note that `src` segment is not included by default, and the path starts on the root of your working directory.

If you pass an absolute path (e.g. "/path/to/controllers/"), it will replace the artifact's default directory totally.
Otherwise, it will be concatenated to the default directory as a child segment:

```bash
hubbitos-cli g controller "user" --output "/src/custom/path"
# generates ./src/custom/path/users_controller.rs
# and adds `users_controller` module to ./src/custom/path/mod.rs
```

```bash
hubbitos-cli g controller "user" --output "child/directory"
# generates ./src/infra/http/controllers/child/directory/users_controller.rs
# and adds `users_controller` module to ./src/infra/http/controllers/child/directory/mod.rs
```

Note that the CLI won't ever create the segments for you. It assumes that they already exist, and will trigger an error
in case they don't. The same goes for the `mod.rs` files inside these directories.
