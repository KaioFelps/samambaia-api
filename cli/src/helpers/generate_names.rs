use crate::error::HubbitosCliError;

pub struct FormattedNames {
    pub capitalized: String,
    pub filename: String,
    pub module: String,
}

pub fn extract_formatted_names(
    args: &[String],
    artifact: &str,
    use_plural: bool,
) -> Result<FormattedNames, HubbitosCliError> {
    let name_argument = match args.get(3) {
        None => {
            return Err(HubbitosCliError::ArgumentError(format!(
                "Missing the {} name argument.",
                artifact
            )))
        }
        Some(name) => name,
    };

    let artifact_name_partials: Vec<&str> = name_argument.split(" ").collect();

    let artifact_capitalized_name = get_capitalized_name(&artifact_name_partials);
    let artifact_snake_cased_name = artifact_name_partials.join("_");

    // e.g.: "user", "controller" => "users_controller.rs"
    let (artifact_module_name, artifact_file_name) = match use_plural {
        true => (
            format!("{}s_{}", artifact_snake_cased_name, artifact),
            format!("{}s_{}.rs", artifact_snake_cased_name, artifact),
        ),
        false => (
            format!("{}_{}", artifact_snake_cased_name, artifact),
            format!("{}_{}.rs", artifact_snake_cased_name, artifact),
        ),
    };

    Ok(FormattedNames {
        capitalized: artifact_capitalized_name,
        module: artifact_module_name,
        filename: artifact_file_name,
    })
}

pub fn get_capitalized_name(repository_name: &Vec<&str>) -> String {
    let mut repository_capitalized_name = String::new();

    for name in repository_name {
        let name_chars: Vec<char> = name.chars().collect();
        let (first_char, name_chars) = name_chars.split_first().unwrap();

        repository_capitalized_name.push(first_char.to_ascii_uppercase());

        for c in name_chars {
            repository_capitalized_name.push(*c);
        }
    }

    repository_capitalized_name
}
