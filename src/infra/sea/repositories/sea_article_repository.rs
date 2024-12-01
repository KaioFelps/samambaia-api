use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder, QuerySelect, QueryTrait};
use std::error::Error;
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::slug::Slug;
use crate::domain::repositories::article_repository::{
    ArticleQueryType, ArticleRepositoryTrait, FindManyArticlesResponse,
};
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::article::Column as ArticleColumn;
use entities::article::Entity as ArticleEntity;

pub struct SeaArticleRepository<'a> {
    sea_service: &'a SeaService,
}

impl SeaArticleRepository<'_> {
    // constructor
    pub async fn new(sea_service: &SeaService) -> SeaArticleRepository<'_> {
        SeaArticleRepository { sea_service }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository<'_> {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let new_article = SeaArticleMapper::article_to_sea_active_model(article);

        let db = &self.sea_service.db;

        let created_article = new_article.insert(db).await?;
        let created_article = SeaArticleMapper::model_to_article(created_article);

        Ok(created_article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>> {
        let article = ArticleEntity::find_by_id(id)
            .one(&self.sea_service.db)
            .await?;

        if article.is_none() {
            return Ok(None);
        }

        let mapped_article = SeaArticleMapper::model_to_article(article.unwrap());

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

        let mapped_article = SeaArticleMapper::model_to_article(article.unwrap());

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
            .apply_if(
                params.clone().query,
                |#[allow(unused_mut)] mut query_builder, query| {
                    self.find_many_get_filters(query_builder, query)
                },
            )
            .apply_if(show_only_approved_state, |query_builder, approved| {
                query_builder.filter(ArticleColumn::Approved.eq(approved))
            })
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let articles_count = ArticleEntity::find()
            .apply_if(
                params.query,
                |#[allow(unused_mut)] mut query_builder, query| {
                    self.find_many_get_filters(query_builder, query)
                },
            )
            .apply_if(show_only_approved_state, |query_builder, approved| {
                query_builder.filter(ArticleColumn::Approved.eq(approved))
            })
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mut articles: Vec<Article> = vec![];

        for article in articles_response.into_iter() {
            articles.push(SeaArticleMapper::model_to_article(article));
        }

        Ok(FindManyArticlesResponse(articles, articles_count))
    }

    async fn get_home_articles(&self) -> Result<Vec<Article>, Box<dyn Error>> {
        let articles = ArticleEntity::find()
            .limit(3)
            .order_by_desc(ArticleColumn::CreatedAt)
            .all(&self.sea_service.db)
            .await?;

        let mut mapped_articles: Vec<Article> = vec![];

        for article in articles {
            mapped_articles.push(SeaArticleMapper::model_to_article(article));
        }

        Ok(mapped_articles)
    }

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let article_id = &article.id().clone();

        let article = SeaArticleMapper::article_to_sea_active_model(article);

        let article = ArticleEntity::update(article.clone())
            .filter(ArticleColumn::Id.eq(*article_id))
            .exec(&self.sea_service.db)
            .await?;

        Ok(SeaArticleMapper::model_to_article(article))
    }
}

impl SeaArticleRepository<'_> {
    fn find_many_get_filters(
        &self,
        #[allow(unused_mut)] mut query_builder: sea_orm::Select<ArticleEntity>,
        query: ArticleQueryType,
    ) -> sea_orm::Select<ArticleEntity> {
        match query {
            ArticleQueryType::Author(content) => {
                let filter = ArticleColumn::AuthorId.eq(content);
                query_builder.filter(filter)
            }
            ArticleQueryType::Title(content) => {
                let filter = Expr::expr(Func::lower(Expr::col(ArticleColumn::Title)))
                    .like(format!("%{}%", content.to_lowercase()));
                query_builder.filter(filter)
            }
            ArticleQueryType::Tag(tag_id) => query_builder.filter(ArticleColumn::TagId.eq(tag_id)),
        }
    }
}
