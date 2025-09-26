use core::panic;

use async_trait::async_trait;
use serde::Deserialize;
use url::Url;

use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

#[derive(Deserialize)]
struct UserMinimalData {
    figure: String,
}

#[async_trait]
pub trait ImagerProvider {
    async fn get_image_base_url(&self, nickname: &str) -> Result<Url, SamambaiaError>;
}

pub struct HabbletImagerProvider<'this> {
    api_url: &'this str,
    imager_url: &'this str,
}

pub struct HabbliveImagerProvider<'this> {
    imager_url: &'this str,
}

impl<'this> HabbletImagerProvider<'this> {
    pub fn new(api_url: &'this str, imager_url: &'this str) -> Self {
        Self {
            api_url,
            imager_url,
        }
    }
}

impl<'this> HabbliveImagerProvider<'this> {
    pub fn new(imager_url: &'this str) -> Self {
        Self { imager_url }
    }
}

#[async_trait]
impl<'this> ImagerProvider for HabbletImagerProvider<'this> {
    async fn get_image_base_url(&self, nickname: &str) -> Result<Url, SamambaiaError> {
        let response = reqwest::get(&format!("{}/player/{nickname}", self.api_url))
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Failed to fetch user data on Habblet API",
                    Box::new(err),
                )
            })?;

        let response = response.json::<UserMinimalData>().await.map_err(|err| {
            generate_service_internal_error(
                "Failed to serialize user data fetched from Habblet API",
                Box::new(err),
            )
        })?;

        let mut url = Url::parse(self.imager_url).unwrap_or_else(|err| {
            panic!("The given Habblet Imager URL set in Hotels Config is invalid, change it in configs crate: {err}");
        });

        url.set_query(Some(&format!("figure={}", response.figure)));
        Ok(url)
    }
}

#[async_trait]
impl<'this> ImagerProvider for HabbliveImagerProvider<'this> {
    async fn get_image_base_url(&self, nickname: &str) -> Result<Url, SamambaiaError> {
        let mut url = Url::parse(self.imager_url).unwrap_or_else(|err| {
            panic!("The given Habblive Imager URL set in Hotels Config is invalid, change it in configs crate: {err}");
        });

        url.set_query(Some(&format!("user={nickname}")));
        Ok(url)
    }
}
