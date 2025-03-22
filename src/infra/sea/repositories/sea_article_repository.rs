use std::error::Error;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::article::{Column as ArticleColumn, Entity as ArticleEntity};
use entities::user::Column as UserColumn;
use migration::extension::postgres::PgExpr;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    PaginatorTrait,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    QueryTrait,
};
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::article_preview::{
    ArticlePreview,
    ArticlePreviewAuthor,
    ArticlePreviewTag,
};
use crate::domain::domain_entities::slug::Slug;
use crate::domain::repositories::article_repository::{
    ArticleQueryType,
    ArticleRepositoryTrait,
    FindManyArticlesPreviewsResponse,
    FindManyArticlesResponse,
};
use crate::error::SamambaiaError;
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaArticleRepository<'a> {
    sea_service: &'a SeaService,
}

impl SeaArticleRepository<'_> {
    // constructor
    pub fn new(sea_service: &SeaService) -> SeaArticleRepository<'_> {
        SeaArticleRepository { sea_service }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository<'_> {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let new_article = SeaArticleMapper::entity_into_active_model(article);

        let db = &self.sea_service.db;

        let created_article = new_article.insert(db).await?;
        let created_article = SeaArticleMapper::model_into_entity(created_article);

        Ok(created_article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>> {
        let article = ArticleEntity::find_by_id(id)
            .one(&self.sea_service.db)
            .await?;

        if article.is_none() {
            return Ok(None);
        }

        let mapped_article = SeaArticleMapper::model_into_entity(article.unwrap());

        Ok(Some(mapped_article))
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, Box<dyn Error>> {
        let article = ArticleEntity::find()
            .filter(ArticleColumn::Slug.eq(slug.to_string()))
            .one(&self.sea_service.db)
            .await?;

        if article.is_none() {
            return Ok(None);
        }

        let mapped_article = SeaArticleMapper::model_into_entity(article.unwrap());

        Ok(Some(mapped_article))
    }

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesResponse, Box<dyn Error>> {
        #[allow(unused_mut)]
        let mut articles_response;

        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        articles_response = ArticleEntity::find()
            .order_by_desc(ArticleColumn::CreatedAt)
            .apply_if(params.query.as_ref(), |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .apply_if(show_only_approved_state, |query_builder, approved| {
                query_builder.filter(ArticleColumn::Approved.eq(approved))
            })
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let articles_count = ArticleEntity::find()
            .apply_if(params.query.as_ref(), |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .apply_if(show_only_approved_state, |query_builder, approved| {
                query_builder.filter(ArticleColumn::Approved.eq(approved))
            })
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mut articles: Vec<Article> = vec![];

        for article in articles_response.into_iter() {
            articles.push(SeaArticleMapper::model_into_entity(article));
        }

        Ok(FindManyArticlesResponse(articles, articles_count))
    }

    async fn find_many_previews(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesPreviewsResponse, Box<dyn Error>> {
        type CustomQueryResult = (
            Uuid,
            String,
            String,
            String,
            String,
            bool,
            Option<i32>,
            Option<String>,
            NaiveDateTime,
            Uuid,
            String,
        );

        let offset = ((params.page - 1) * params.items_per_page) as u64;

        let (articles, articles_count): (Vec<CustomQueryResult>, u64) = tokio::try_join!(
            ArticleEntity::find()
                .select_only()
                .columns([
                    ArticleColumn::Id,
                    ArticleColumn::Slug,
                    ArticleColumn::Title,
                    ArticleColumn::CoverUrl,
                    ArticleColumn::Description,
                    ArticleColumn::Approved,
                    ArticleColumn::TagId,
                    ArticleColumn::TagValue,
                    ArticleColumn::CreatedAt,
                ])
                .column(UserColumn::Id)
                .column(UserColumn::Nickname)
                .inner_join(entities::user::Entity)
                .order_by_desc(ArticleColumn::CreatedAt)
                .apply_if(params.query.as_ref(), |builder, query| {
                    self.find_many_get_filters(builder, query)
                })
                .apply_if(show_only_approved_state, |builder, approved| {
                    builder.filter(ArticleColumn::Approved.eq(approved))
                })
                .limit(params.items_per_page as u64)
                .offset(offset)
                .into_tuple()
                .all(&self.sea_service.db),
            ArticleEntity::find()
                .apply_if(params.query.as_ref(), |query_builder, query| {
                    self.find_many_get_filters(query_builder, query)
                })
                .apply_if(show_only_approved_state, |query_builder, approved| {
                    query_builder.filter(ArticleColumn::Approved.eq(approved))
                })
                .offset(offset)
                .count(&self.sea_service.db)
        )
        .map_err(Box::new)
        .map_err(|err| err as Box<dyn Error>)?;

        let mut parsed_articles = Vec::with_capacity(params.items_per_page as usize);

        for (
            article_id,
            article_slug,
            article_title,
            article_cover_url,
            article_description,
            article_approved,
            tag_id,
            tag_value,
            article_created_at,
            author_id,
            author_nickname,
        ) in articles
        {
            let tag = {
                if tag_id.is_some() ^ tag_value.is_some() {
                    return Err(Box::new(SamambaiaError::internal_err().with_message(
                        "Found an article instance that has a tag that misses either id or value",
                    )));
                }
                if tag_id.is_none() {
                    None
                } else {
                    Some(ArticlePreviewTag::new(tag_id.unwrap(), tag_value.unwrap()))
                }
            };

            let author = ArticlePreviewAuthor::new(author_id, author_nickname);

            parsed_articles.push(ArticlePreview::new(
                article_id,
                article_cover_url,
                article_title,
                article_description,
                article_approved,
                tag,
                author,
                article_created_at,
                Slug::new_from_existing(article_slug),
            ))
        }

        Ok(FindManyArticlesPreviewsResponse(
            parsed_articles,
            articles_count,
        ))
    }

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let article_id = &article.id().clone();

        let article = SeaArticleMapper::entity_into_active_model(article);

        let article = ArticleEntity::update(article.clone())
            .filter(ArticleColumn::Id.eq(*article_id))
            .exec(&self.sea_service.db)
            .await?;

        Ok(SeaArticleMapper::model_into_entity(article))
    }
}

impl SeaArticleRepository<'_> {
    fn find_many_get_filters(
        &self,
        query_builder: sea_orm::Select<ArticleEntity>,
        query: &ArticleQueryType,
    ) -> sea_orm::Select<ArticleEntity> {
        match query {
            ArticleQueryType::Author(content) => {
                let filter = ArticleColumn::AuthorId.eq(*content);
                query_builder.filter(filter)
            }
            ArticleQueryType::Title(content) => {
                // let filter = Expr::expr(Func::lower(Expr::col(ArticleColumn::Title)))
                // .ilike(format!("%{}%", content.to_lowercase()));
                query_builder
                    .filter(Expr::col(ArticleColumn::Title).ilike(format!("%{}%", content)))
            }
            ArticleQueryType::Tag(tag_id) => query_builder.filter(ArticleColumn::TagId.eq(*tag_id)),
        }
    }
}
