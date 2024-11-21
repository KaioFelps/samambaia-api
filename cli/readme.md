# Using Hubbitos CLI for generating shit.

## Enabling the CLI
### Bash
Open your bash and type `source .bashrc` and you good to go with Hubbitos CLI.

### Powershell
Open your powershell terminal and type `. ./src/cli.ps1` and you good to go with Hubbitos CLI.

## Services
Generate a service file on `src/domain/services` and append it in `src/domain/services/mod.rs`.
```bash
hubbitos-cli generate service "service name lowercase splitten by spaces"
```

To overwrite the directory:
```bash
hubbitos-cli generate service "service name" --dir new/path
# not passing the new output directory after --dir flag will error
```

This will generate a service just like:
```rust
use std::error::Error;

pub struct ServiceNameParams {}

pub struct ServiceNameService {}

impl ServiceNameService {
    pub fn new() -> Self {
        ServiceNameService {

        }
    }

    pub async fn exec(&self, params: ServiceNameParams) -> Result<(), Box<dyn Error>> {}
}

#[cfg(test)]
mod test {}
```

## Repository
If for some reason you aim to generate only a repository with no entity, the command below will generate a new file on `src/domain/repositories` and append it to `src/domain/repositories/mod.rs`.
```bash
hubbitos-cli generate repository "repository name"
```

To overwrite the target directory, pass the `--dir` flag followed by the desired directory. "src" is not included by default and the path starts on "/" directory.
```bash
hubbitos-cli generate repository "repository name" --dir "src/custom/path"
# not passing the new output directory after --dir flag will error
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
# you can also overwrite the default directory with --dir flag
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
