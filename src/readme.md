# Using Hubbitos CLI for generating shit.

## Enabling the CLI
### Bash
Open your bash and type `source .bashrc` and you good to go with Hubbitos CLI.

### Powershell
Open your powershell terminal and type `. /src/cli.ps1` and you good to go with Hubbitos CLI.

## Services
Generate a service file on `src/domain/services` and append it in `src/domain/services/mod.rs`.
```bash
hubbitos-cli generate service "service name lowercase splitten by spaces"
```

To overwrite the directory:
```bash
hubbitos-cli generate service "service name" --dir new/path
# not passing the new output directory after --dir flag will panic
```

This will generate a service just like:
```rs
use std::error::Error;

pub struct ServiceNameParams {}

pub struct ServiceNameService {}

impl ServiceNameService {
    pub fn new() -> Self {
        ServiceNameService {

        }
    }

    pub async fn exec(&self, params: ServiceNameParams) -> Result<(), Box<dyn Error>> {}
}

#[cfg(test)]
mod test {}
```
