use entities::article::{ActiveModel as ArticleActiveModel, Model as ArticleModel};
use entities::article_tag::{ActiveModel as ArticleTagActiveModel, Model as ArticleTagModel};
use sea_orm::IntoActiveValue;

use super::sea_article_tag_mapper::SeaArticleTagMapper;
use super::SeaMapper;
use crate::domain::domain_entities::article::Article;
use crate::domain::value_objects::slug::Slug;

pub struct SeaArticleMapper;

impl
    SeaMapper<
        Article,
        (ArticleModel, Vec<ArticleTagModel>),
        (ArticleActiveModel, Vec<ArticleTagActiveModel>),
    > for SeaArticleMapper
{
    fn entity_into_model(entity: Article) -> (ArticleModel, Vec<ArticleTagModel>) {
        (
            ArticleModel {
                id: entity.id(),
                author_id: entity.author_id(),
                cover_url: entity.cover_url().to_owned(),
                title: entity.title().to_owned(),
                content: entity.content().to_owned(),
                approved: entity.approved(),
                created_at: entity.created_at(),
                updated_at: entity.updated_at(),
                slug: entity.slug().to_string(),
                description: entity.description().to_string(),
            },
            entity
                .get_tags()
                .into_iter()
                .cloned()
                .map(SeaArticleTagMapper::entity_into_model)
                .collect::<Vec<_>>(),
        )
    }

    fn entity_into_active_model(
        entity: Article,
    ) -> (ArticleActiveModel, Vec<ArticleTagActiveModel>) {
        (
            ArticleActiveModel {
                id: entity.id().into_active_value(),
                author_id: entity.author_id().into_active_value(),
                cover_url: entity.cover_url().to_owned().into_active_value(),
                title: entity.title().to_owned().into_active_value(),
                content: entity.content().to_owned().into_active_value(),
                approved: entity.approved().into_active_value(),
                created_at: entity.created_at().into_active_value(),
                updated_at: entity.updated_at().into_active_value(),
                slug: entity.slug().to_string().into_active_value(),
                description: entity.description().to_owned().into_active_value(),
            },
            entity
                .get_tags()
                .into_iter()
                .cloned()
                .map(SeaArticleTagMapper::entity_into_active_model)
                .collect::<Vec<_>>(),
        )
    }

    fn active_model_into_entity(
        (active_model, active_tags): (ArticleActiveModel, Vec<ArticleTagActiveModel>),
    ) -> Article {
        Article::new_from_existing(
            active_model.id.unwrap(),
            active_model.author_id.unwrap(),
            active_model.cover_url.unwrap(),
            active_model.title.unwrap(),
            active_model.content.unwrap(),
            active_model.approved.unwrap(),
            active_model.created_at.unwrap(),
            active_model.updated_at.unwrap(),
            Slug::new_from_existing(active_model.slug.unwrap()),
            active_model.description.unwrap(),
            active_tags
                .into_iter()
                .map(SeaArticleTagMapper::active_model_into_entity)
                .collect::<Vec<_>>(),
        )
    }

    fn model_into_entity((model, tags): (ArticleModel, Vec<ArticleTagModel>)) -> Article {
        Article::new_from_existing(
            model.id,
            model.author_id,
            model.cover_url,
            model.title,
            model.content,
            model.approved,
            model.created_at,
            model.updated_at,
            Slug::new_from_existing(model.slug),
            model.description,
            tags.into_iter()
                .map(SeaArticleTagMapper::model_into_entity)
                .collect::<Vec<_>>(),
        )
    }
}
