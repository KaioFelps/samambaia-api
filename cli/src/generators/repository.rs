use std::path::Path;

use crate::error::SamambaiaCliError;
use crate::helpers::generate_names::extract_formatted_names;
use crate::helpers::resolve_dir_path::resolve_dir_path;
use crate::helpers::save_artifact::save_artifact;
use crate::templates::get_repository_flat_template;
use crate::DEFAULT_REPOSITORIES_DIR;

pub fn generate_repository(args: &[String], current_dir: &Path) -> Result<(), SamambaiaCliError> {
    let repository_name = extract_formatted_names(args, "repository", true)?;

    let output_dir = resolve_dir_path(args, DEFAULT_REPOSITORIES_DIR)?;
    let output_dir_path = current_dir.join(Path::new(&output_dir));

    if let Err(err) = std::fs::create_dir_all(&output_dir_path) {
        return Err(SamambaiaCliError::GeneratorError(format!(
            "Error on creating provided directory: {}",
            err,
        )));
    };

    save_artifact(
        "repository",
        &output_dir_path,
        &repository_name,
        get_repository_flat_template(&repository_name.capitalized).as_bytes(),
    )
}
