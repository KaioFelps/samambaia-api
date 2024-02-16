#[derive(Debug, PartialEq, Eq)]
pub struct PaginationResponse {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u64,
}

#[derive(Clone)]
pub struct PaginationParameters<QT> {
    pub page: u32,
    pub items_per_page: u32,
    pub query: Option<QT>,
}