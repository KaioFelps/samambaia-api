pub fn get_service_template(service_capitalized_name: &String) -> String {
    format!(
"use std::error::Error;

pub struct {service_capitalized_name}Params {{}}

pub struct {service_capitalized_name}Service {{}}

impl {service_capitalized_name}Service {{
    pub fn new() -> Self {{
        {service_capitalized_name}Service {{}}
    }}

    pub async fn exec(&self, params: {service_capitalized_name}Params) -> Result<(), Box<dyn Error>> {{
        Ok(())
    }}
}}

#[cfg(test)]
mod test {{}}
"
    )
}
