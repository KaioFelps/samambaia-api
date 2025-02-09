use std::path::Path;

use crate::error::SamambaiaCliError;
use crate::helpers::generate_names::extract_formatted_names;
use crate::helpers::resolve_dir_path::resolve_dir_path;
use crate::helpers::save_artifact::save_artifact;
use crate::templates::get_service_template;
use crate::DEFAULT_SERVICES_DIR;

pub fn generate_service(args: &[String], current_dir: &Path) -> Result<(), SamambaiaCliError> {
    let service_name = extract_formatted_names(args, "service", false)?;

    let output_dir = resolve_dir_path(args, DEFAULT_SERVICES_DIR)?;
    let output_dir_path = current_dir.join(Path::new(&output_dir));

    if let Err(err) = std::fs::create_dir_all(&output_dir_path) {
        return Err(SamambaiaCliError::GeneratorError(format!(
            "Error on creating provided directory: {}",
            err,
        )));
    };

    save_artifact(
        "service",
        &output_dir_path,
        &service_name,
        get_service_template(&service_name.capitalized).as_bytes(),
    )
}
