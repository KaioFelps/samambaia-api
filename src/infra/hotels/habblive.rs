use async_trait::async_trait;
use serde::Deserialize;

use crate::domain::external_services::hotel_utilities_external_services::HotelUtilitiesExternalService;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

#[derive(Deserialize)]
struct UserMinimalData {
    motto: Option<String>,
    error: Option<String>,
}

pub struct HabbliveUtilitiesExternalServices<'this> {
    api_url: &'this str,
}

impl<'this> HabbliveUtilitiesExternalServices<'this> {
    pub fn new(api_url: &'this str) -> Self {
        Self { api_url }
    }
}

#[async_trait]
impl<'this> HotelUtilitiesExternalService for HabbliveUtilitiesExternalServices<'this> {
    async fn get_user_motto(&self, nickname: &str) -> Result<Option<String>, SamambaiaError> {
        let response = reqwest::get(&format!("{}/player/{}", self.api_url, nickname))
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Failed to fetch user data on Habblive API",
                    Box::new(err),
                )
            })?;

        let user_info = response.json::<UserMinimalData>().await.map_err(|err| {
            generate_service_internal_error(
                "Habblive API has not returned a valid JSON on user data fetch",
                Box::new(err),
            )
        })?;

        if user_info.error.is_some() {
            return Ok(None);
        }

        Ok(user_info.motto)
    }
}
