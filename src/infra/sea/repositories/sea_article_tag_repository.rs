use std::error::Error;

use async_trait::async_trait;
use entities::article_tag::{Column as ArticleTagColumn, Entity as ArticleTagEntity};
use migration::{Expr, Func};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    Condition,
    ConnectionTrait,
    EntityTrait,
    IntoActiveValue,
    PaginatorTrait,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    QueryTrait,
    Select,
};
use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
use crate::domain::repositories::article_tag_repository::{
    ArticleTagQueryType,
    ArticleTagRepositoryTrait,
    FindManyArticleTagsResponse,
};
use crate::error::SamambaiaError;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::mappers::sea_article_tag_mapper::SeaArticleTagMapper;
use crate::util::generate_service_internal_error;

pub struct SeaArticleTagRepository<'a, C: ConnectionTrait> {
    db: &'a C,
}

impl<'a, C: ConnectionTrait> SeaArticleTagRepository<'a, C> {
    // constructor
    pub fn new(connection: &'a C) -> Self {
        SeaArticleTagRepository { db: connection }
    }
}

#[async_trait]
impl<C: ConnectionTrait> ArticleTagRepositoryTrait for SeaArticleTagRepository<'_, C> {
    async fn create(&self, article_tag: DraftArticleTag) -> Result<ArticleTag, Box<dyn Error>> {
        let new_article_tag = SeaArticleTagMapper::draft_entity_into_active_model(article_tag);

        let created_article_tag = new_article_tag.insert(self.db).await?;
        let created_article_tag = SeaArticleTagMapper::model_into_entity(created_article_tag);

        Ok(created_article_tag)
    }

    async fn find_by_id(&self, article_tag_id: i32) -> Result<Option<ArticleTag>, Box<dyn Error>> {
        let article_tag = ArticleTagEntity::find_by_id(article_tag_id)
            .one(self.db)
            .await?;

        match article_tag {
            None => Ok(None),
            Some(article_tag) => Ok(Some(SeaArticleTagMapper::model_into_entity(article_tag))),
        }
    }

    async fn find_by_value(
        &self,
        article_tag_value: String,
    ) -> Result<Option<ArticleTag>, Box<dyn Error>> {
        let article_tag = ArticleTagEntity::find()
            .filter(ArticleTagColumn::Value.eq(article_tag_value))
            .one(self.db)
            .await?;

        match article_tag {
            None => Ok(None),
            Some(article_tag) => Ok(Some(SeaArticleTagMapper::model_into_entity(article_tag))),
        }
    }

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleTagQueryType>,
    ) -> Result<FindManyArticleTagsResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let filter = |query_builder: Select<ArticleTagEntity>, query: ArticleTagQueryType| {
            let ArticleTagQueryType::Value(query) = query;
            let filter = Expr::expr(Func::lower(Expr::col(ArticleTagColumn::Value)))
                .like(format!("%{}%", query.to_lowercase()));
            query_builder.filter(filter)
        };

        let article_tags_response = ArticleTagEntity::find()
            .order_by_desc(ArticleTagColumn::Id)
            .apply_if(params.clone().query, filter)
            .limit(items_per_page)
            .offset(leap)
            .all(self.db)
            .await?;

        let article_tags_count = ArticleTagEntity::find()
            .apply_if(params.query, filter)
            .offset(leap)
            .count(self.db)
            .await?;

        let mut article_tags: Vec<ArticleTag> = vec![];

        for article_tag in article_tags_response.into_iter() {
            article_tags.push(SeaArticleTagMapper::model_into_entity(article_tag));
        }

        Ok(FindManyArticleTagsResponse(
            article_tags,
            article_tags_count,
        ))
    }

    async fn save(&self, article_tag: ArticleTag) -> Result<ArticleTag, Box<dyn Error>> {
        let active_article_tag = SeaArticleTagMapper::entity_into_active_model(article_tag.clone());

        let _ = ArticleTagEntity::update(active_article_tag)
            .filter(ArticleTagColumn::Id.eq(article_tag.id()))
            .exec(self.db)
            .await?;

        Ok(article_tag)
    }

    async fn delete(&self, article_tag: ArticleTag) -> Result<(), Box<dyn Error>> {
        let article_tag = SeaArticleTagMapper::entity_into_active_model(article_tag);

        ArticleTagEntity::delete(article_tag).exec(self.db).await?;

        Ok(())
    }

    async fn find_many_by_ids(&self, tag_ids: Vec<i32>) -> Result<Vec<ArticleTag>, SamambaiaError> {
        Ok(ArticleTagEntity::find()
            .filter(ArticleTagColumn::Id.is_in(tag_ids)).all(self.db)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on `SeaArticleTagRepository::find_many_by_ids`, on fetching article tags by ids",
                    Box::new(err))})?
            .into_iter()
            .map(SeaArticleTagMapper::model_into_entity).collect())
    }

    async fn associate_tags_to_article(
        &self,
        article_id: Uuid,
        tags_ids: Vec<i32>,
    ) -> Result<(), SamambaiaError> {
        if tags_ids.is_empty() {
            return Ok(());
        }

        let added = tags_ids
            .into_iter()
            .map(|tag| entities::articles_tags_rel::ActiveModel {
                article_id: article_id.into_active_value(),
                tag_id: tag.into_active_value(),
            })
            .collect::<Vec<_>>();

        entities::articles_tags_rel::Entity::insert_many(added)
            .exec(self.db)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred in `SeaArticleTagRepository::associate_tags_to_article` when inserting many article-tag relationships",
                Box::new(err)))?;

        Ok(())
    }

    async fn disassociate_tags_from_article(
        &self,
        article_id: Uuid,
        tags_ids: Vec<i32>,
    ) -> Result<(), SamambaiaError> {
        if tags_ids.is_empty() {
            return Ok(());
        }

        let mut removed = Condition::any();

        for tag in tags_ids.into_iter() {
            removed = removed.add(
                Condition::all()
                    .add(entities::articles_tags_rel::Column::TagId.eq(tag))
                    .add(entities::articles_tags_rel::Column::ArticleId.eq(article_id)),
            )
        }

        entities::articles_tags_rel::Entity::delete_many()
            .filter(removed)
            .exec(self.db)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred in `SeaArticleTagRepository::associate_tags_to_article` when deleting many article-tag relationships",
                Box::new(err)))?;

        Ok(())
    }
}
