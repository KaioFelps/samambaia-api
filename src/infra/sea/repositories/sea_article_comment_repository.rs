use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, TransactionTrait, Value,
};
use std::error::Error;
use std::future::Future;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_comment_repository::{
    ArticleCommentRepositoryTrait, CommentQueryType, FindManyCommentsResponse,
};
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::mappers::sea_comment_mapper::SeaCommentMapper;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment::Column as CommentColumn;
use entities::comment::Entity as CommentEntity;

use entities::article::Entity as ArticleEntity;

pub struct SeaArticleCommentRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SeaArticleCommentRepository<'a> {
    pub async fn new(service: &'a SeaService) -> Self {
        SeaArticleCommentRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl ArticleCommentRepositoryTrait for SeaArticleCommentRepository<'_> {
    async fn find_many_comments(
        &self,
        article_id: Option<Uuid>,
        include_inactive: bool,
        params: PaginationParameters<CommentQueryType>,
    ) -> Result<FindManyCommentsResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let include_inactive = Some(include_inactive);

        let comments = CommentEntity::find()
            .apply_if(include_inactive, |query_builder, val| {
                if !val {
                    query_builder.filter(CommentColumn::IsActive.eq(true))
                } else {
                    query_builder
                }
            })
            .apply_if(article_id, |query_builder, id| {
                query_builder.filter(CommentColumn::ArticleId.eq(id))
            })
            .apply_if(params.clone().query, |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .order_by_desc(CommentColumn::CreatedAt)
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let comments_count = CommentEntity::find()
            .apply_if(include_inactive, |query_builder, val| {
                if !val {
                    query_builder.filter(CommentColumn::IsActive.eq(true))
                } else {
                    query_builder
                }
            })
            .apply_if(article_id, |query_builder, id| {
                query_builder.filter(CommentColumn::ArticleId.eq(id))
            })
            .apply_if(params.clone().query, |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mut mapped_comments: Vec<Comment> = vec![];

        for comment in comments {
            mapped_comments.push(SeaCommentMapper::model_into_entity(comment))
        }

        Ok(FindManyCommentsResponse(mapped_comments, comments_count))
    }

    async fn delete_many_comments_by_article_id(
        &self,
        article_id: Uuid,
    ) -> Result<(), Box<dyn Error>> {
        self.delete_all_articles_comments(&self.sea_service.db, article_id)
            .await?;

        Ok(())
    }

    async fn inactivate_many_comments_by_article_id(
        &self,
        article_id: Uuid,
    ) -> Result<(), Box<dyn Error>> {
        self.inactivate_all_articles_comments(&self.sea_service.db, article_id)
            .await?;

        Ok(())
    }

    async fn delete_article_with_comments(&self, article: Article) -> Result<(), Box<dyn Error>> {
        let article_id = article.id();

        let article = SeaArticleMapper::entity_into_active_model(article);

        let transaction = self.sea_service.db.begin().await?;

        ArticleEntity::delete(article).exec(&transaction).await?;

        self.delete_all_articles_comments(&transaction, article_id)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn delete_article_and_inactivate_comments(
        &self,
        article: Article,
    ) -> Result<(), Box<dyn Error>> {
        let article_id = article.id();

        let article = SeaArticleMapper::entity_into_active_model(article);

        let transaction = self.sea_service.db.begin().await?;

        self.inactivate_all_articles_comments(&transaction, article_id)
            .await?;

        ArticleEntity::delete(article).exec(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl SeaArticleCommentRepository<'_> {
    fn delete_all_articles_comments<'lf, C: ConnectionTrait>(
        &self,
        conn: &'lf C,
        article_id: Uuid,
    ) -> impl Future<Output = Result<sea_orm::DeleteResult, sea_orm::DbErr>> + 'lf {
        CommentEntity::delete_many()
            .filter(CommentColumn::ArticleId.eq(article_id))
            .exec(conn)
    }

    fn inactivate_all_articles_comments<'lf, C: ConnectionTrait>(
        &self,
        conn: &'lf C,
        article_id: Uuid,
    ) -> impl Future<Output = Result<sea_orm::UpdateResult, sea_orm::DbErr>> + 'lf {
        CommentEntity::update_many()
            .col_expr(CommentColumn::ArticleId, Expr::value(Value::Uuid(None)))
            .col_expr(
                CommentColumn::IsActive,
                Expr::value(Value::Bool(Some(false))),
            )
            .filter(CommentColumn::ArticleId.eq(article_id))
            .exec(conn)
    }

    fn find_many_get_filters(
        &self,
        query_builder: sea_orm::Select<CommentEntity>,
        query: CommentQueryType,
    ) -> sea_orm::Select<CommentEntity> {
        match query {
            CommentQueryType::Author(content) => {
                let filter = CommentColumn::AuthorId.eq(content);

                query_builder.filter(filter)
            }
            CommentQueryType::Content(content) => {
                let filter = Expr::expr(Func::lower(Expr::col(CommentColumn::Content)))
                    .like(format!("%{}%", content.to_lowercase()));
                query_builder.filter(filter)
            }
        }
    }
}
