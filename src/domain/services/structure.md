## Template for new services

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