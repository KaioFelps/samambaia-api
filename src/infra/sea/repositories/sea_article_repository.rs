use async_trait::async_trait;
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use std::error::Error;

use crate::domain::repositories::article_repositoy::ArticleRepositoryTrait;
use crate::domain::domain_entities::article::Article;
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::article::Entity as ArticleEntity;
use entities::article::Column as ArticleColumn;

pub struct SeaArticleRepository {
    sea_service: SeaService
}

impl SeaArticleRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaArticleRepository {
            sea_service: service.clone()
        }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let new_article = SeaArticleMapper::article_to_sea_active_model(article);

        let db = &self.sea_service.db;

        let created_article = new_article.insert(db).await.unwrap();
        let created_article = SeaArticleMapper::model_to_article(created_article);

        Ok(created_article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>> {
        let article = ArticleEntity::find_by_id(id).one(&self.sea_service.db).await;

        match article {
            Ok(article) => {
                if article.is_none() {
                    return Ok(None);
                }

                let mapped_article = SeaArticleMapper::model_to_article(article.unwrap());


                Ok(Some(mapped_article))
            },
            Err(err) => Err(Box::new(err))
        }
    }

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>> {
        let article_id = &article.id().clone();

        let article = SeaArticleMapper::article_to_sea_active_model(article);

        let res = ArticleEntity::update(article.clone())
        .filter(ArticleColumn::Id.eq(*article_id))
        .exec(&self.sea_service.db)
        .await;

        match res {
            Ok(article) => Ok(SeaArticleMapper::model_to_article(article)),
            Err(err) => Err(Box::new(err))
        }
    }
}

