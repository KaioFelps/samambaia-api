use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize)]
pub struct ShowArticleQueryDto {
    #[serde(rename = "commentsPage")]
    pub comments_page: Option<u32>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ArticlesArchiveQueryDto {
    #[validate(range(min = 1, message = "A página deve ser maior ou igual a 1."))]
    pub page: Option<i32>,

    pub title: Option<String>,

    pub author: Option<String>,

    #[serde(rename = "includingTags")]
    #[validate(custom(function = validate_vec_range))]
    pub including_tags: Option<Vec<i32>>,
}

fn validate_vec_range(numbers: &Vec<i32>) -> Result<(), ValidationError> {
    for &num in numbers {
        if num < 1 {
            let mut error = ValidationError::new("range_out_of_bounds");
            error.message = Some("IDs de tags não podem ser menores que 1.".into());
            return Err(error);
        }
    }
    Ok(())
}
