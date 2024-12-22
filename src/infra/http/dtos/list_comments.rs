use serde::{Deserialize, Serialize};
use validator::Validate;

/**
* Since it's a query DTO, we let it in snake_case
* because it is the most common way of passing parameters in a search string
* # Example
* ie: ?per_page=1&include_inactive=true
*/
#[derive(Serialize, Deserialize, Validate)]
pub struct ListCommentsDto {
    #[validate(range(min = 1, message = "Min page is 1."))]
    pub page: Option<u32>,

    #[validate(range(min = 1, message = "Per page amount mus be at least 1."))]
    pub per_page: Option<u8>,

    pub include_inactive: Option<bool>,

    pub content: Option<String>,

    pub author: Option<String>,
}
