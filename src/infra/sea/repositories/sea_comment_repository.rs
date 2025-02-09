use std::error::Error;

use async_trait::async_trait;
use entities::comment::Entity as CommentEntity;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::infra::sea::mappers::sea_comment_mapper::SeaCommentMapper;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaCommentRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SeaCommentRepository<'a> {
    // constructor
    pub fn new(service: &'a SeaService) -> Self {
        SeaCommentRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl CommentRepositoryTrait for SeaCommentRepository<'_> {
    async fn create(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
        let active_comment = SeaCommentMapper::entity_into_active_model(comment);
        let model_comment = active_comment.insert(&self.sea_service.db).await?;

        let comment = SeaCommentMapper::model_into_entity(model_comment);

        Ok(comment)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>> {
        Ok(CommentEntity::find_by_id(id)
            .one(&self.sea_service.db)
            .await?
            .map(SeaCommentMapper::model_into_entity))
    }

    async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        SeaCommentMapper::entity_into_model(comment)
            .delete(&self.sea_service.db)
            .await?;

        Ok(())
    }

    async fn save(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
        let comment = SeaCommentMapper::entity_into_active_model(comment)
            .update(&self.sea_service.db)
            .await?;

        Ok(SeaCommentMapper::model_into_entity(comment))
    }
}
