use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct PatchArticleApprovedDto {
    #[validate(required(message = "Approved is a mandatory field."))]
    pub approved: Option<bool>,
}
