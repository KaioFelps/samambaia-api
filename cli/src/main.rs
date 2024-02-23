use std::io::Write;
use std::path::PathBuf;
use std::fs::{self, OpenOptions};

fn main() {
    let current_dir = std::env::current_dir().unwrap();
            
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        return;
    }

    let generate_aliases = [
        "g",
        "gen",
        "generate"
    ];

    if generate_aliases.contains(&&args[1][..]) {
        let generators = ["service"];

        if args.len() < 3 || !generators.contains(&&args[2][..]) {
            return;
        }

        if args[2] == "service" {
            let service_name_arg = &args[3];
            let service_name: Vec<&str> = service_name_arg.split(" ").collect();

            let mut service_capitalized_name: Vec<String> = Vec::new();
            let mut service_file_name: Vec<String> = Vec::new();

            for name in service_name {
                service_file_name.push(name.to_string());

                let name_chars: Vec<char> = name.chars().collect();
                let (first_char, name_chars) = name_chars.split_first().unwrap();

                service_capitalized_name.push(first_char.to_ascii_uppercase().to_string());
                service_capitalized_name.push(name_chars.iter().collect());
            }

            let service_capitalized_name = service_capitalized_name.join("");
            let mut service_file_name = service_file_name.join("_");
            service_file_name.push_str("_service");

            let services_dir: PathBuf;

            if args.contains(&"--dir".to_string()) {
                let mut arg_index = None;

                for (i, arg) in args.iter().enumerate() {
                    if arg == "--dir" {
                        if args.len() < i + 1 {
                            eprintln!("Error: --dir flag requires to be followed by the new output directory.");
                            return;
                        }

                        arg_index = Some(i + 1);
                        break;
                    }
                }

                let arg_index = arg_index.unwrap();

                let custom_service_dir;

                if args[arg_index].starts_with("/") {
                    custom_service_dir = args[arg_index][1..].to_string();
                }
                else {
                    custom_service_dir = args[arg_index].clone();
                }

                services_dir = current_dir.join(custom_service_dir);
            } else {
                services_dir = current_dir.join("src/domain/services/");
            }

            let service_file = services_dir.join(format!("{}.rs", service_file_name));

            let _ = fs::write(service_file, format!(
r#"use std::error::Error;

pub struct {service_capitalized_name}Params {{}}

pub struct {service_capitalized_name}Service {{}}

impl {service_capitalized_name}Service {{
    pub fn new() -> Self {{
        {service_capitalized_name}Service {{}}
    }}

    pub async fn exec(&self, params: {service_capitalized_name}Params) -> Result<(), Box<dyn Error>> {{}}
}}

#[cfg(test)]
mod test {{}}
"#
            ));

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
                        eprintln!("Error on adding service {}.rs to mod.rs: {}", service_file_name, err)
                    }
                }
            }
        }
    };
}