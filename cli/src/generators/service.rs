use std::{fs::OpenOptions, io::Write, path::{Path, PathBuf}};

use crate::{helpers::{extract_dir_flag, get_capitalized_name}, templates::get_service_template};

pub fn generate_service(args: &Vec<String>, current_dir: &PathBuf) {
    let service_name_arg = &args[3];
    let service_name: Vec<&str> = service_name_arg.split(" ").collect();

    let service_capitalized_name = get_capitalized_name::exec(&service_name);
    let mut service_file_name = service_name.join("_");
    service_file_name.push_str("_service");

    let services_dir: PathBuf;

    let custom_final_path = extract_dir_flag::exec(args);

    if custom_final_path.is_err() {
        eprintln!("Error: {}.", custom_final_path.unwrap_err().message());
        return;
    }
    
    let custom_final_path = custom_final_path.unwrap();

    match custom_final_path {
        Some(dir) => services_dir = current_dir.join(Path::new(&dir)),
        None => services_dir = current_dir.join("src/domain/services/")
    }
    
    if let Err(err) = std::fs::create_dir_all(services_dir.clone()) {
        eprintln!("Error on creating provided directory: {}", err);
        return;
    };

    let service_file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(services_dir.join(format!("{}.rs", service_file_name)));

    match service_file {
        Err(err) => {
            eprintln!("Error on creating {}.rs service: {}",service_file_name, err);
            return;
        },

        Ok(mut file) => {
            if let Err(err) = file.write(get_service_template(&service_capitalized_name).as_bytes()) {
                eprintln!("Error on creating {}.rs: {}",service_file_name, err);
                return;
            }

            println!("Created service {}.rs on {}.", service_file_name, services_dir.display().to_string());
        }
    };

    let services_mod_file = OpenOptions::new().create(true).append(true).open(services_dir.join("mod.rs"));

    match services_mod_file {
        Err(err) => {
            eprintln!("Error on adding service {}.rs to mod.rs: {}", service_file_name, err)
        },

        Ok(mut file) => {
            if let Err(err) = file.write_all(
                format!("pub mod {};\r\n", service_file_name)
                .as_bytes()
            ) {
                eprintln!("Error on adding service {}.rs to mod.rs: {}", service_file_name, err);
                return;
            }

            println!("Added service {}.rs to mod.rs.", service_file_name);
        }
    }
}
