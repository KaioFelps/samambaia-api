use crate::error::HubbitosCliError;

pub fn exec(args: &[String]) -> Result<Option<String>, HubbitosCliError> {
    if args.contains(&"-o".to_string()) {
        let mut arg_index = None;
        let output_flag_aliases = ["-o", "--output", "--output-dir", "--dir", "-d"];

        for (i, arg) in args.iter().enumerate() {
            if output_flag_aliases.contains(&arg.as_str()) {
                if args.len() < i + 1 {
                    return Err(HubbitosCliError::ArgumentError(format!(
                        "'{}' flag requires to be followed by the new output directory.",
                        arg
                    )));
                }

                arg_index = Some(i + 1);
                break;
            }
        }

        let arg_index = arg_index.unwrap();

        let custom_repository_dir = match args[arg_index].starts_with("/") {
            true => args[arg_index][1..].to_string(),
            false => args[arg_index].clone(),
        };

        return Ok(Some(custom_repository_dir));
    }

    Ok(None)
}
