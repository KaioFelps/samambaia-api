#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FetchArticleQuery {
    Title(String),
    Author(String),
    Tags(Vec<i32>),
}

pub struct FetchArticlesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub queries: Vec<FetchArticleQuery>,
    pub approved_state: Option<bool>,
}
