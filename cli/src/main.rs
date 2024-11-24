use std::io;

use error::{HubbitosCliError, IntoIoError};

mod error;
mod generators;
mod helpers;
mod templates;

static DEFAULT_CONTROLLERS_DIR: &str = "src/infra/http/controllers/";
static DEFAULT_REPOSITORIES_DIR: &str = "src/domain/repositories/";
static DEFAULT_SERVICES_DIR: &str = "src/domain/services/";

fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir().unwrap();
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        return Err(HubbitosCliError::ArgumentError(
            "Invalid command. 1 or less arguments have been passed. Please, refer to the documentation.".into(),
        )
        .into_io_err());
    }

    let generate_aliases = ["g", "gen", "generate"];

    if generate_aliases.contains(&&args[1][..]) {
        let generators = ["service", "repository", "controller"];

        if args.len() < 3 || !generators.contains(&&args[2][..]) {
            return Err(HubbitosCliError::ArgumentError(
                "Not any valid generator has been specified. Cancelling the operation.".into(),
            )
            .into_io_err());
        }

        return match args[2].as_str() {
            "service" => generators::service(&args, &current_dir).map_err(IntoIoError::into_io_err),
            "repository" => {
                generators::repository(&args, &current_dir).map_err(IntoIoError::into_io_err)
            }
            "controller" => {
                generators::controller(&args, &current_dir).map_err(IntoIoError::into_io_err)
            }
            _ => Err(HubbitosCliError::ArgumentError(format!(
                "Invalid command --{} provided: {}.",
                args[1], args[2]
            ))
            .into_io_err()),
        };
    }

    Err(
        HubbitosCliError::ArgumentError(format!("Invalid command provided: {}.", args[1]))
            .into_io_err(),
    )
}
