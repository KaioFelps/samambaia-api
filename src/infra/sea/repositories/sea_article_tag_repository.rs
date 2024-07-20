use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Select};
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::error::Error;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article_tag::DraftArticleTag;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::infra::sea::mappers::sea_article_tag_mapper::SeaArticleTagMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::article_tag::Entity as ArticleTagEntity;
use entities::article_tag::Column as ArticleTagColumn;
use crate::domain::repositories::article_tag_repository::{ArticleTagQueryType, ArticleTagRepositoryTrait, FindManyArticleTagsResponse};

pub struct SeaArticleTagRepository {
    sea_service: SeaService,
}

impl SeaArticleTagRepository {
    // constructor
    pub async fn new(service: SeaService) -> Self {
        SeaArticleTagRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl ArticleTagRepositoryTrait for SeaArticleTagRepository {
    async fn create(&self, article_tag: DraftArticleTag) -> Result<ArticleTag, Box<dyn Error>> {
        let new_article_tag = SeaArticleTagMapper::draft_article_tag_to_sea_active_model(article_tag);

        let db = &self.sea_service.db;

        let created_article_tag = new_article_tag.insert(db).await?;
        let created_article_tag = SeaArticleTagMapper::model_to_article_tag(created_article_tag);

        Ok(created_article_tag)
    }

    async fn find_by_id(&self, article_tag_id: i32) -> Result<Option<ArticleTag>, Box<dyn Error>> {
        let article_tag = ArticleTagEntity::
        find_by_id(article_tag_id)
            .one(&self.sea_service.db)
            .await?;

        match article_tag {
            None => Ok(None),
            Some(article_tag) => {
                Ok( Some( SeaArticleTagMapper::model_to_article_tag(article_tag) ) )
            }
        }
    }

    async fn find_by_value(&self, article_tag_value: String) -> Result<Option<ArticleTag>, Box<dyn Error>> {
        let article_tag = ArticleTagEntity::find()
            .filter(ArticleTagColumn::Value.eq(article_tag_value))
            .one(&self.sea_service.db)
            .await?;

        match article_tag {
            None => Ok(None),
            Some(article_tag) => {
                Ok( Some( SeaArticleTagMapper::model_to_article_tag(article_tag) ) )
            }
        }
    }

    async fn find_many(&self, params: PaginationParameters<ArticleTagQueryType>) -> Result<FindManyArticleTagsResponse, Box<dyn Error>> {
        let article_tags_response;

        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;
        
        let filter = |query_builder: Select<ArticleTagEntity>, query: ArticleTagQueryType| {
            let ArticleTagQueryType::Title(query) = query;
            let filter = Expr::expr(Func::lower(Expr::col(ArticleTagColumn::Value))).like(format!("%{}%", query.to_lowercase()));
            query_builder.filter(filter)
        };

        article_tags_response = ArticleTagEntity::find()
            .order_by_desc(ArticleTagColumn::Id)
            .apply_if(params.clone().query, filter.clone())
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db).await?;

        let article_tags_count = ArticleTagEntity::find()
            .apply_if(params.query, filter)
            .offset(leap)
            .count(&self.sea_service.db).await?;

        let mut article_tags: Vec<ArticleTag> = vec![];

        for article_tag in article_tags_response.into_iter() {
            article_tags.push(SeaArticleTagMapper::model_to_article_tag(article_tag));
        }

        Ok(FindManyArticleTagsResponse(article_tags, article_tags_count))
    }

    async fn save(&self, article_tag: ArticleTag) -> Result<ArticleTag, Box<dyn Error>> {
        let comm_rep_id = article_tag.id();

        let article_tag = SeaArticleTagMapper::article_tag_to_sea_active_model(article_tag);

        let article_tag = ArticleTagEntity
        ::update(article_tag)
            .filter(ArticleTagColumn::Id.eq(comm_rep_id))
            .exec(&self.sea_service.db)
            .await?;

        let article_tag = SeaArticleTagMapper::model_to_article_tag(article_tag);

        Ok(article_tag)
    }

    async fn delete(&self, article_tag: ArticleTag) -> Result<(), Box<dyn Error>> {
        let article_tag = SeaArticleTagMapper::article_tag_to_sea_active_model(article_tag);

        ArticleTagEntity
        ::delete(article_tag)
            .exec(&self.sea_service.db).await?;

        Ok(())
    }
}
