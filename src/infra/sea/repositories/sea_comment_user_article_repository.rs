use std::error::Error;

use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait};
use uuid::Uuid;

use crate::core::pagination::{PaginationParameters, Query};
use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
use crate::domain::repositories::comment_user_article_repository::{CommentUserArticleRepositoryTrait, CommentWithAuthorQueryType, FindManyCommentsWithAuthorResponse};
use crate::infra::sea::mappers::sea_comment_with_author_mapper::SeaCommentWithAuthorMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment::Entity as CommentEntity;
use entities::comment::Column as CommentColumn;

use entities::user::Entity as UserEntity;

pub struct SeaCommentUserArticleRepository {
    sea_service: SeaService,
}

impl SeaCommentUserArticleRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaCommentUserArticleRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl CommentUserArticleRepositoryTrait for SeaCommentUserArticleRepository {
    async fn find_many_comments(
        &self,
        article_id: Uuid,
        params: PaginationParameters<CommentWithAuthorQueryType>
    ) -> Result<FindManyCommentsWithAuthorResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = ((&current_page - 1) * items_per_page) as u64;

        let comments = CommentEntity::find()
        .filter(CommentColumn::ArticleId.eq(article_id))
        .apply_if(params.clone().query, |query_builder, query| self.find_many_get_filters(query_builder, query))
        .order_by_desc(CommentColumn::CreatedAt)
        .find_also_related(UserEntity)
        .limit(items_per_page)
        .offset(leap)
        .all(&self.sea_service.db)
        .await?;

        let comments_count = CommentEntity::find()
        .filter(CommentColumn::ArticleId.eq(article_id))
        .apply_if(params.clone().query, |query_builder, query| self.find_many_get_filters(query_builder, query))
        .offset(leap)
        .count(&self.sea_service.db)
        .await?;

        let mut mapped_comments: Vec<CommentWithAuthor> = vec![];

        for models in comments {
            mapped_comments.push(
                SeaCommentWithAuthorMapper::model_to_comment_with_author(
                    (models.0, models.1.unwrap())
                )
            )
        }

        Ok(FindManyCommentsWithAuthorResponse (mapped_comments, comments_count))
    }
}

impl SeaCommentUserArticleRepository {
    fn find_many_get_filters(&self, query_builder: sea_orm::Select<CommentEntity>, query: Query<CommentWithAuthorQueryType>) -> sea_orm::Select<CommentEntity> {
        let content = query.content;

        match query.query_type {
            CommentWithAuthorQueryType::AUTHOR => {
                let content = Uuid::parse_str(&content).unwrap();

                let filter = CommentColumn::AuthorId.eq(content);
    
                query_builder.filter(filter)
            },
            CommentWithAuthorQueryType::CONTENT => {
                let filter = Expr::expr(Func::lower(Expr::col(CommentColumn::Content))).like(format!("%{}%", content));
                query_builder.filter(filter)
            }
        }
    }
}