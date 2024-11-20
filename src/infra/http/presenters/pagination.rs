use serde::{Deserialize, Serialize};

use crate::core::pagination::PaginationResponse;

#[derive(Serialize, Deserialize)]
pub struct MappedPagination {
    #[serde(rename = "currentPage")]
    current_page: u32,
    #[serde(rename = "totalItems")]
    total_items: u64,
    #[serde(rename = "totalPages")]
    total_pages: u32,
    #[serde(rename = "itemsPerPage")]
    items_per_page: u8,
}

pub struct PaginationPresenter;

impl PaginationPresenter {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_http(pagination_details: PaginationResponse, per_page: u8) -> MappedPagination {
        MappedPagination {
            current_page: pagination_details.current_page,
            total_items: pagination_details.total_items,
            total_pages: pagination_details.total_pages,
            items_per_page: per_page,
        }
    }
}
