use std::error::Error;
use std::future::Future;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait};
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_comment_repository::{ArticleCommentRepositoryTrait, FindManyCommentsResponse};
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::mappers::sea_comment_mapper::SeaCommentMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment::Entity as CommentEntity;
use entities::comment::Column as CommentColumn;

use entities::article::Entity as ArticleEntity;

pub struct SeaArticleCommentRepository {
    sea_service: SeaService,
}

impl SeaArticleCommentRepository {
    pub fn new(service: SeaService) -> Self {
        SeaArticleCommentRepository {
            sea_service: service
        }
    }
}

#[async_trait]
impl ArticleCommentRepositoryTrait for SeaArticleCommentRepository {
    async fn find_many_comments_by_article_id(&self, article_id: Uuid) -> Result<FindManyCommentsResponse, Box<dyn Error>> {
        let comments = CommentEntity::find()
        .filter(CommentColumn::ArticleId.eq(article_id))
        .order_by_desc(CommentColumn::CreatedAt)
        .all(&self.sea_service.db)
        .await?;

        let comments_count = CommentEntity::find()
        .filter(CommentColumn::ArticleId.eq(article_id))
        .count(&self.sea_service.db)
        .await?;

        let mut mapped_comments: Vec<Comment> = vec![];

        for comment in comments {
            mapped_comments.push(
                SeaCommentMapper::model_to_comment(comment)
            )
        }

        Ok(FindManyCommentsResponse (mapped_comments, comments_count))
    }

    async fn delete_many_comments_by_article_id(&self, article_id: Uuid) -> Result<(), Box<dyn Error>> {
        self.delete_all_articles_comment(&self.sea_service.db, article_id).await?;

        Ok(())
    }

    async fn delete_article_with_comments(&self, article: Article) -> Result<(), Box<dyn Error>> {
        let article_id = article.id();

        let article = SeaArticleMapper::article_to_sea_active_model(article);

        let transaction = self.sea_service.db.begin().await?;

        ArticleEntity::delete(article)
        .exec(&transaction)
        .await?;

        self.delete_all_articles_comment(&transaction, article_id).await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl SeaArticleCommentRepository {
    fn delete_all_articles_comment<'lf, C: ConnectionTrait>
    (&self, conn: &'lf C, article_id: Uuid)
    -> impl Future<Output = Result<sea_orm::DeleteResult, sea_orm::DbErr>> + 'lf{
        CommentEntity::delete_many()
        .filter(CommentColumn::ArticleId.eq(article_id))
        .exec(conn)
    }
}