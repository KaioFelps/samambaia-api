use crate::configs::app::APP_CONFIG;
use crate::domain::external_services::hotel_utilities_external_services::HotelUtilitiesExternalService;
use crate::error::SamambaiaError;

pub struct VerifyVerificationCodeParams<'this> {
    pub nickname: &'this str,
}

pub struct VerifyVerificationCodeService {
    hotel_external_services: Box<dyn HotelUtilitiesExternalService>,
}

pub struct VerifyVerificationCodeResponse {
    pub is_authorized: bool,
    pub motto: Option<String>,
}

impl VerifyVerificationCodeService {
    pub fn new(hotel_external_services: Box<dyn HotelUtilitiesExternalService>) -> Self {
        Self {
            hotel_external_services,
        }
    }

    pub async fn execute(
        &self,
        params: VerifyVerificationCodeParams<'_>,
    ) -> Result<VerifyVerificationCodeResponse, SamambaiaError> {
        let user_motto = self
            .hotel_external_services
            .get_user_motto(params.nickname)
            .await?;

        let user_motto = match user_motto {
            Some(motto) => motto,
            None => {
                return Ok(VerifyVerificationCodeResponse {
                    is_authorized: false,
                    motto: None,
                });
            }
        };

        Ok(VerifyVerificationCodeResponse {
            is_authorized: user_motto == APP_CONFIG.verification_motto,
            motto: Some(user_motto),
        })
    }
}
