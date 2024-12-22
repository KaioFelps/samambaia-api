use std::{fs::OpenOptions, io::Write, path::Path};

use crate::error::SamambaiaCliError;

use super::generate_names::FormattedNames;

pub fn save_artifact(
    artifact: &str,
    output_dir: &Path,
    artifact_name: &FormattedNames,
    template: &[u8],
) -> Result<(), SamambaiaCliError> {
    let artifact_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_dir.join(artifact_name.filename.clone()));

    match artifact_file {
        Err(err) => {
            return Err(SamambaiaCliError::GeneratorError(format!(
                "Error on creating {}: {}",
                artifact_name.filename, err
            )))
        }

        Ok(mut file) => {
            if let Err(err) = file.write(template) {
                return Err(SamambaiaCliError::GeneratorError(format!(
                    "Error on creating {}: {}",
                    artifact_name.filename, err
                )));
            }

            println!(
                "Created {} {} in {}.",
                artifact,
                artifact_name.filename,
                output_dir.display()
            );
        }
    };

    let artifacts_dir_mod_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_dir.join("mod.rs"));

    match artifacts_dir_mod_file {
        Err(err) => Err(SamambaiaCliError::GeneratorError(format!(
            "Error on adding {} {} to mod.rs: {}",
            artifact, artifact_name.filename, err
        ))),

        Ok(mut file) => {
            if let Err(err) =
                file.write_all(format!("pub mod {};\r\n", artifact_name.module).as_bytes())
            {
                return Err(SamambaiaCliError::GeneratorError(format!(
                    "Error on adding {} {} to mod.rs: {}",
                    artifact, artifact_name.filename, err
                )));
            }

            println!("Added {} {} to mod.rs.", artifact, artifact_name.filename);
            Ok(())
        }
    }
}
