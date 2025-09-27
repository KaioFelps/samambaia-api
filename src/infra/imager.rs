use std::collections::HashMap;

use crate::error::SamambaiaError;
use crate::infra::imager::providers::ImagerProvider;

pub mod factory;
pub mod providers;

pub struct Imager {
    provider: Box<dyn ImagerProvider>,
}

impl Imager {
    pub fn new(provider: Box<dyn ImagerProvider>) -> Self {
        Self { provider }
    }

    pub async fn mount_imager_url(
        &self,
        nickname: &str,
        params: HashMap<String, String>,
    ) -> Result<String, SamambaiaError> {
        let mut base_url = self.provider.get_image_base_url(nickname).await?;

        {
            let mut search_params = base_url.query_pairs_mut();
            params.into_iter().for_each(|(key, value)| {
                search_params.append_pair(&key, &value);
            });
        }

        Ok(base_url.to_string())
    }
}
