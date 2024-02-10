use std::error::Error;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, IntoActiveValue, ModelTrait, TransactionTrait};
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::infra::sea::mappers::sea_comment_mapper::SeaCommentMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment::Entity as CommentEntity;

use entities::comment_article::ActiveModel as CommentArticleActiveModel;
use entities::comment_article::Entity as CommentArticleEntity;
use entities::comment_article::Column as CommentArticleColumn;

pub struct SeaCommentRepository {
    sea_service: SeaService,
}

impl SeaCommentRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaCommentRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl CommentRepositoryTrait for SeaCommentRepository {
    async fn create(&self, comment: Comment, article_id: Uuid) -> Result<Comment, Box<dyn Error>> {
        let transaction = self.sea_service.db.begin().await?;

        let active_comment = SeaCommentMapper::comment_to_sea_active_model(comment);
        let model_comment = active_comment.insert(&transaction).await?;

        let comment = SeaCommentMapper::model_to_comment(model_comment);
        
        CommentArticleActiveModel {
            id: Uuid::new_v4().into_active_value(),
            article_id: article_id.into_active_value(),
            comment_id: comment.id().into_active_value(),
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(comment)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>> {
        let comment = CommentEntity::find_by_id(id).one(&self.sea_service.db).await?;

        match comment {
            None => Ok(None),
            Some(comment) => {
                let mapped_comment = SeaCommentMapper::model_to_comment(comment);
                Ok(Some(mapped_comment))
            }
        }
    }

    // async fn find_many_by_article_id(&self, article_id: Uuid) -> Result<FindManyCommentsResponse, Box<dyn Error>>;

    async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>> {
        let relation = CommentArticleEntity::find()
        .filter(CommentArticleColumn::CommentId.eq(comment.id()))
        .one(&self.sea_service.db)
        .await?.unwrap();

        let comment = SeaCommentMapper::comment_to_sea_model(comment);

        let transaction = self.sea_service.db.begin().await?;

        comment.delete(&transaction).await?;
        
        relation.delete(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

}
