use std::error::Error;

use async_trait::async_trait;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ModelTrait};
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::infra::sea::mappers::sea_comment_mapper::SeaCommentMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment::Entity as CommentEntity;

pub struct SeaCommentRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SeaCommentRepository<'a> {
    // constructor
    pub async fn new(service: &'a SeaService) -> Self {
        SeaCommentRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl CommentRepositoryTrait for SeaCommentRepository<'_> {
    async fn create(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
        let active_comment = SeaCommentMapper::comment_to_sea_active_model(comment);
        let model_comment = active_comment.insert(&self.sea_service.db).await?;

        let comment = SeaCommentMapper::model_to_comment(model_comment);

        Ok(comment)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>> {
        let comment = CommentEntity::find_by_id(id)
            .one(&self.sea_service.db)
            .await?;

        match comment {
            None => Ok(None),
            Some(comment) => {
                let mapped_comment = SeaCommentMapper::model_to_comment(comment);
                Ok(Some(mapped_comment))
            }
        }
    }

    async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        let comment = SeaCommentMapper::comment_to_sea_model(comment);

        comment.delete(&self.sea_service.db).await?;

        Ok(())
    }

    async fn save(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
        let comment = SeaCommentMapper::comment_to_sea_active_model(comment);

        let comment = comment.update(&self.sea_service.db).await?;

        Ok(SeaCommentMapper::model_to_comment(comment))
    }
}
