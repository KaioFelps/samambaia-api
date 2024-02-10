use std::error::Error;

use async_trait::async_trait;
use log::debug;
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
    async fn create(&self, comment: Comment, article_id: Uuid) -> Result<Comment, Box<dyn Error>> {
        let transaction = self.sea_service.db.begin().await?;

        let active_comment = SeaCommentMapper::comment_to_sea_active_model(comment);
        let model_comment = active_comment.insert(&transaction).await;

        if model_comment.is_err().clone() {
            debug!("\r\n============\r\nComment insert failed with the err: {:#?}.\r\n============\r\n", model_comment.as_ref().unwrap_err());
        }

        let comment = SeaCommentMapper::model_to_comment(model_comment.unwrap());
        
        let comment_article_insert = CommentArticleActiveModel {
            id: Uuid::new_v4().into_active_value(),
            article_id: article_id.into_active_value(),
            comment_id: comment.id().into_active_value(),
        }
        .insert(&transaction)
        .await;

        if comment_article_insert.is_err() {
            debug!("\r\n============\r\nComment_Article insert failed with the err: {:#?}.\r\n============\r\n", comment_article_insert.unwrap_err());
        }
    
        let res = transaction.commit().await;

        if res.is_err() {
            debug!("===========\r\nError occurred on sea_comment_repository.rs, line 50:\r\n{:#?}\r\n===========", res.unwrap_err());
            return Err(Box::new( InternalError::new() ) );
        }

        Ok(comment)
    }

    // async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, Box<dyn Error>>;

    // async fn find_many_by_comment_id(&self, comment_id: Uuid) -> Result<FindManyCommentsResponse, Box<dyn Error>>;

    // async fn delete(&self, comment: Comment) -> Result<(), Box<dyn Error>>;

}
