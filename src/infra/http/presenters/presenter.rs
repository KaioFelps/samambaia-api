use crate::infra::http::presenters::pagination::MappedPagination;
use serde::{Deserialize, Serialize};

pub trait PresenterTrait<Entity, MappedEntity> {
    #[allow(clippy::wrong_self_convention)]
    fn to_http(entity: Entity) -> MappedEntity;

    #[allow(clippy::wrong_self_convention)]
    fn to_json_paginated_wrapper(
        entities: Vec<MappedEntity>,
        pagination: MappedPagination,
    ) -> JsonWrappedPaginatedEntity<MappedEntity> {
        JsonWrappedPaginatedEntity {
            pagination,
            data: entities,
        }
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
