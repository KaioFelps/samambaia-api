use std::error::Error;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, IntoActiveValue, TransactionTrait};
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::errors::internal_error::InternalError;
use crate::infra::sea::mappers::sea_comment_mapper::SeaCommentMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment_article::ActiveModel as CommentArticleActiveModel;

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
    // async fn create(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
    //     let new_comment = SeaCommentMapper::comment_to_sea_active_model(comment);

    //     let db = &self.sea_service.db;

    //     let created_comment = new_comment.insert(db).await.unwrap();
    //     let created_comment = SeaCommentMapper::model_to_comment(created_comment);

    //     Ok(created_comment)
    // }

    async fn create(&self, comment: Comment, article_id: Uuid) -> Result<Comment, Box<dyn Error>> {
        let transaction = self.sea_service.db.begin().await?;

        let active_comment = SeaCommentMapper::comment_to_sea_active_model(comment);
        let model_comment = active_comment.insert(&transaction).await?;

        let comment = SeaCommentMapper::model_to_comment(model_comment);

        CommentArticleActiveModel {
            article_id: article_id.into_active_value(),
            comment_id: comment.id().into_active_value(),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;

        let res = transaction.commit().await;

        if res.is_err() {
            return Err(Box::new( InternalError::new() ) );
        }

        Ok(comment)
    }

    // async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>>;

    // async fn find_many_by_comment_id(&self, comment_id: Uuid) -> Result<FindManyCommentsResponse, Box<dyn Error>>;

    // async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>>;

}
