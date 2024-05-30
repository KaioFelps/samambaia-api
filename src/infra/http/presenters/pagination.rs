use serde::{Deserialize, Serialize};

use crate::core::pagination::PaginationResponse;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MappedPagination {
    currentPage: u32,
    totalItems: u64,
    totalPages: u32,
    itemsPerPage: u8
}

pub struct PaginationPresenter;

impl PaginationPresenter {
    pub fn to_http(pagination_details: PaginationResponse, per_page: u8) -> MappedPagination {
        MappedPagination {
            currentPage: pagination_details.current_page,
            totalItems: pagination_details.total_items,
            totalPages: pagination_details.total_pages,
            itemsPerPage: per_page
        }
    }
}
