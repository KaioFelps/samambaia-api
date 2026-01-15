use core::panic;
use std::borrow::Cow;
use std::ops::Deref;
use std::time::Duration;

use async_trait::async_trait;
use moka::future::{Cache, CacheBuilder};
use reqwest::Client;
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
    http_client: Client,
    cache: Cache<String, Cow<'static, str>>,
}

pub struct HabbliveImagerProvider<'this> {
    imager_url: &'this str,
}

impl<'this> HabbletImagerProvider<'this> {
    pub fn new(api_url: &'this str, imager_url: &'this str) -> Result<Self, SamambaiaError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let fake_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(fake_user_agent),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred when instantiating `HabbletImagerProvider`",
                    Box::new(err),
                )
            })?;

        let cache = CacheBuilder::new(10_000)
            .name("habblet user's figures")
            .time_to_live(Duration::from_mins(15))
            .time_to_idle(Duration::from_mins(5))
            .build();

        Ok(Self {
            api_url,
            imager_url,
            http_client,
            cache,
        })
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
        let user_figure = match self.cache.get(nickname).await {
            Some(cached_figure) => cached_figure,
            None => {
                let response = self
                    .http_client
                    .get(format!("{}/player/{nickname}", self.api_url))
                    .send()
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

                let figure = Cow::<'static, str>::Owned(response.figure);

                self.cache
                    .insert(nickname.to_string(), figure.clone())
                    .await;

                figure
            }
        };

        let mut url = Url::parse(self.imager_url).unwrap_or_else(|err| {
            panic!("The given Habblet Imager URL set in Hotels Config is invalid, change it in configs crate: {err}");
        });

        url.set_query(Some(&format!("figure={}", user_figure.deref())));
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
