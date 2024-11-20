#[derive(Debug)]
pub struct ExtractDirDlagError {
    message: String,
}

impl ExtractDirDlagError {
    pub fn new() -> Self {
        Self {
            message: "'--dir' flag requires to be followed by the new output directory."
                .to_string(),
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

pub fn exec(args: &[String]) -> Result<Option<String>, ExtractDirDlagError> {
    if args.contains(&"--dir".to_string()) {
        let mut arg_index = None;

        for (i, arg) in args.iter().enumerate() {
            if arg == "--dir" {
                if args.len() < i + 1 {
                    return Err(ExtractDirDlagError::new());
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
