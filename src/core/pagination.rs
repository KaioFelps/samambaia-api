#[derive(Debug, PartialEq, Eq)]
pub struct PaginationResponse {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum QueryType {
    TITLE,
    AUTHOR,
}

#[derive(Clone)]
pub struct Query {
    pub content: String,
    pub query_type: QueryType
}

#[derive(Clone)]
pub struct PaginationParameters {
    pub page: u32,
    pub items_per_page: u32,
    pub query: Option<Query>,
}