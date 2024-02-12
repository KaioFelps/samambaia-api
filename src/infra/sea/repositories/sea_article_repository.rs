use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryOrder, QuerySelect, QueryTrait};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use std::error::Error;

use crate::core::pagination::{PaginationParameters, Query, QueryType};
use crate::domain::repositories::article_repository::{ArticleRepositoryTrait, FindManyResponse};
use crate::domain::domain_entities::article::Article;
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::article::Entity as ArticleEntity;
use entities::article::Column as ArticleColumn;

pub struct SeaArticleRepository {
    sea_service: SeaService,
}

impl SeaArticleRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaArticleRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let new_article = SeaArticleMapper::article_to_sea_active_model(article);

        let db = &self.sea_service.db;

        let created_article = new_article.insert(db).await?;
        let created_article = SeaArticleMapper::model_to_article(created_article);

        Ok(created_article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>> {
        let article = ArticleEntity::find_by_id(id).one(&self.sea_service.db).await?;

        if article.is_none() {
            return Ok(None);
        }

        let mapped_article = SeaArticleMapper::model_to_article(article.unwrap());

        Ok(Some(mapped_article))
    }

    async fn find_many(&self, params: PaginationParameters) -> Result<FindManyResponse, Box<dyn Error>> {
        #[allow(unused_mut)]
        let mut articles_response;

        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = ((&current_page - 1) * items_per_page) as u64;

        articles_response = ArticleEntity::find()
        .order_by_desc(ArticleColumn::CreatedAt)
        .apply_if(params.clone().query, |#[allow(unused_mut)] mut query_builder, query| self.find_many_get_filters(query_builder, query))
        .limit(items_per_page)
        .offset(leap)
        .all(&self.sea_service.db).await?;

        let articles_count = ArticleEntity::find()
        .apply_if(params.query, |#[allow(unused_mut)] mut query_builder, query| self.find_many_get_filters(query_builder, query))
        .offset(leap)
        .count(&self.sea_service.db).await?;

        let mut articles: Vec<Article> = vec![];

        for article in articles_response.into_iter() {
            articles.push(SeaArticleMapper::model_to_article(article));
        }

        Ok(FindManyResponse(articles, articles_count))
    }

    async fn get_home_articles(&self) -> Result<Vec<Article>, Box<dyn Error>> {
        let articles = ArticleEntity::find()
        .limit(3)
        .order_by_desc(ArticleColumn::CreatedAt)
        .all(&self.sea_service.db)
        .await?;

        let mut mapped_articles: Vec<Article> = vec![];

        for article in articles {
            mapped_articles.push(
                SeaArticleMapper::model_to_article(article)
            );
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

    async fn delete(&self, article: Article) -> Result<(), Box<dyn Error>> {
        let article = SeaArticleMapper::article_to_sea_active_model(article);

        ArticleEntity::delete(article)
        .exec(&self.sea_service.db)
        .await?;

        Ok(())
    }
}

impl SeaArticleRepository {
    fn find_many_get_filters(&self, #[allow(unused_mut)] mut query_builder: sea_orm::Select<ArticleEntity>, query: Query) -> sea_orm::Select<ArticleEntity> {
        let content = query.content;

        match query.query_type {
            QueryType::AUTHOR => {
                let content = Uuid::parse_str(&content).unwrap();

                let filter = ArticleColumn::AuthorId.eq(content);
    
                query_builder.filter(filter.clone())
            },
            QueryType::TITLE => {
                let filter = Expr::expr(Func::lower(Expr::col(ArticleColumn::Title))).like(format!("%{}%", content));
                query_builder.filter(filter.clone())
            }
        }
    }
}