use crate::{
    core::pagination::PaginationResponse, infra::http::presenters::pagination::MappedPagination,
};
use serde::{Deserialize, Serialize};

use super::pagination::PaginationPresenter;

pub trait PresenterTrait<Entity, MappedEntity> {
    #[allow(clippy::wrong_self_convention)]
    fn to_http(entity: Entity) -> MappedEntity;

    #[allow(clippy::wrong_self_convention)]
    fn to_json_paginated_wrapper(
        entities: Vec<Entity>,
        pagination: PaginationResponse,
        per_page: u8,
    ) -> JsonWrappedPaginatedEntity<MappedEntity> {
        let data = entities.into_iter().map(Self::to_http).collect::<Vec<_>>();
        let pagination = PaginationPresenter::to_http(pagination, per_page);

        JsonWrappedPaginatedEntity { pagination, data }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonWrappedEntity<MappedEntity> {
    pub data: MappedEntity,
}

#[derive(Serialize, Deserialize)]
pub struct JsonWrappedPaginatedEntity<MappedEntity> {
    pub data: Vec<MappedEntity>,
    pub pagination: MappedPagination,
}
