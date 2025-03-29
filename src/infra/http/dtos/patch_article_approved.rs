use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct PatchArticleApprovedDto {
    #[validate(required(message = "Approved is a mandatory field."))]
    pub approved: Option<bool>,
}
