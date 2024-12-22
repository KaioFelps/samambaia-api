use crate::error::SamambaiaCliError;

use super::extract_dir_flag;

pub fn resolve_dir_path(args: &[String], default: &str) -> Result<String, SamambaiaCliError> {
    let custom_output_dir = extract_dir_flag::exec(args)?;

    if let Some(directory) = custom_output_dir {
        // if its an absolute path, returns it but without the "/" prefix,
        // as it would cause to create the file in the root of the filesystem instead of
        // the working directory
        if let Some(directory) = directory.strip_prefix("/") {
            return Ok(directory.into());
        }

        // assures that the directory ends with /, e.g. "path/to/output/dir/"
        let directory_with_trailling_slash = match directory.strip_suffix("/").is_some() {
            true => directory,
            false => format!("{}/", directory),
        };

        // concatenates the default with the custom directory if it ain't an absolute path
        // e.g. "default/path/the/custom/inner/path/"
        return Ok(format!("{}{}", default, directory_with_trailling_slash));
    }

    Ok(default.into())
}
