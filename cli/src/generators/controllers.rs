use std::path::Path;

use crate::{
    error::HubbitosCliError,
    helpers::{
        generate_names::extract_formatted_names, resolve_dir_path::resolve_dir_path,
        save_artifact::save_artifact,
    },
    templates::get_controller_template,
    DEFAULT_CONTROLLERS_DIR,
};

pub fn generate_controller(args: &[String], current_dir: &Path) -> Result<(), HubbitosCliError> {
    let controller_name = extract_formatted_names(args, "controller")?;

    let output_dir = resolve_dir_path(args, DEFAULT_CONTROLLERS_DIR)?;
    let output_dir_path = current_dir.join(Path::new(&output_dir));

    if let Err(err) = std::fs::create_dir_all(&output_dir_path) {
        return Err(HubbitosCliError::GeneratorError(format!(
            "Error on creating provided directory: {}",
            err,
        )));
    };

    save_artifact(
        "controller",
        &output_dir_path,
        &controller_name,
        get_controller_template(&controller_name.capitalized).as_bytes(),
    )
}
