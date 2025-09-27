use async_trait::async_trait;

use crate::error::SamambaiaError;

#[async_trait]
pub trait HotelUtilitiesExternalService {
    async fn get_user_motto(&self, nickname: &str) -> Result<Option<String>, SamambaiaError>;
}
