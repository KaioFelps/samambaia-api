#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FetchArticleQuery {
    Title(String),
    Author(String),
}

pub struct FetchArticlesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<FetchArticleQuery>,
    pub approved_state: Option<bool>,
}
