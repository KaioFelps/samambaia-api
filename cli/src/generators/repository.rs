use std::{fs::OpenOptions, io::Write, path::Path};

use crate::{
    helpers::{extract_dir_flag, get_capitalized_name},
    templates::get_repository_flat_template,
};

pub fn generate_repository(args: &[String], current_dir: &Path) {
    let repository_name_arg = &args[3];
    let repository_name: Vec<&str> = repository_name_arg.split(" ").collect();

    let repository_capitalized_name = get_capitalized_name::exec(&repository_name);
    let mut repository_file_name = repository_name.join("_");
    repository_file_name.push_str("_repository");

    let custom_final_path = extract_dir_flag::exec(args);

    if custom_final_path.is_err() {
        eprintln!("Error: {}", custom_final_path.unwrap_err().message());
        return;
    }

    let custom_final_path = custom_final_path.unwrap();

    let repositories_dir = match custom_final_path {
        None => current_dir.join("src/domain/repositories/"),
        Some(dir) => current_dir.join(Path::new(&dir)),
    };

    if let Err(err) = std::fs::create_dir_all(repositories_dir.clone()) {
        eprintln!("Error on creating provided directory: {}", err);
        return;
    };

    let repository_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(repositories_dir.join(format!("{}.rs", repository_file_name)));

    match repository_file {
        Err(err) => {
            eprintln!("Error on creating {}.rs: {}", repository_file_name, err);
            return;
        }

        Ok(mut file) => {
            if let Err(err) =
                file.write(get_repository_flat_template(&repository_capitalized_name).as_bytes())
            {
                eprintln!("Error on creating {}.rs: {}", repository_file_name, err);
                return;
            }

            println!(
                "Created repository {}.rs on {}.",
                repository_file_name,
                repositories_dir.display()
            );
        }
    };

    let repositories_mod_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(repositories_dir.join("mod.rs"));

    match repositories_mod_file {
        Err(err) => {
            eprintln!(
                "Error on adding repository {}.rs to mod.rs: {}",
                repository_file_name, err
            )
        }

        Ok(mut file) => {
            if let Err(err) =
                file.write_all(format!("pub mod {};\r\n", repository_file_name).as_bytes())
            {
                eprintln!(
                    "Error on adding repository {}.rs to mod.rs: {}",
                    repository_file_name, err
                );
                return;
            }

            println!("Added repository {}.rs to mod.rs.", repository_file_name);
        }
    }
}
