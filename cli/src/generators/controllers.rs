use std::{fs::OpenOptions, io::Write, path::Path};

use crate::{
    helpers::{extract_dir_flag, get_capitalized_name},
    templates::get_controller_template,
};

pub fn generate_controller(args: &[String], current_dir: &Path) {
    let controller_name_arg = &args[3];
    let controller_name: Vec<&str> = controller_name_arg.split(" ").collect();

    let controller_capitalized_name = get_capitalized_name::exec(&controller_name);
    let mut controller_file_name = controller_name.join("_");
    controller_file_name.push_str("s_controller");

    let custom_final_path = extract_dir_flag::exec(args);

    if custom_final_path.is_err() {
        eprintln!("Error: {}", custom_final_path.unwrap_err().message());
        return;
    }

    let custom_final_path = custom_final_path.unwrap();

    let controllers_dir = match custom_final_path {
        None => current_dir.join("src/infra/http/controllers/"),
        Some(dir) => current_dir.join(Path::new(&dir)),
    };

    if let Err(err) = std::fs::create_dir_all(controllers_dir.clone()) {
        eprintln!("Error on creating provided directory: {}", err);
        return;
    };

    let controller_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(controllers_dir.join(format!("{}.rs", controller_file_name)));

    match controller_file {
        Err(err) => {
            eprintln!("Error on creating {}.rs: {}", controller_file_name, err);
            return;
        }

        Ok(mut file) => {
            if let Err(err) =
                file.write(get_controller_template(&controller_capitalized_name).as_bytes())
            {
                eprintln!("Error on creating {}.rs: {}", controller_file_name, err);
                return;
            }

            println!(
                "Created controller {}.rs on {}.",
                controller_file_name,
                controllers_dir.display()
            );
        }
    };

    let controllers_mod_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(controllers_dir.join("mod.rs"));

    match controllers_mod_file {
        Err(err) => {
            eprintln!(
                "Error on adding controller {}.rs to mod.rs: {}",
                controller_file_name, err
            )
        }

        Ok(mut file) => {
            if let Err(err) =
                file.write_all(format!("pub mod {};\r\n", controller_file_name).as_bytes())
            {
                eprintln!(
                    "Error on adding controller {}.rs to mod.rs: {}",
                    controller_file_name, err
                );
                return;
            }

            println!("Added controller {}.rs to mod.rs.", controller_file_name);
        }
    }
}
