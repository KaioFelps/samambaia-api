#[derive(Debug, PartialEq, Eq)]
pub struct PaginationResponse {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u64,
}

impl PaginationResponse {
    pub fn new(current_page: u32, items_count: u64, per_page: u32) -> Self {
        Self {
            current_page,
            total_items: items_count,
            total_pages: (items_count as f64 / per_page as f64).ceil() as u32,
        }
    }
}

#[derive(Clone)]
pub struct PaginationParameters<QT: Clone> {
    pub page: u32,
    pub items_per_page: u32,
    pub query: Option<QT>,
}

pub const DEFAULT_PER_PAGE: u8 = 9;
