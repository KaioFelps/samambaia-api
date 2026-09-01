use entities::article::{ActiveModel as ArticleActiveModel, Model as ArticleModel};
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::value_objects::slug::Slug;

pub struct SeaArticleMapper;

impl SeaArticleMapper {
    pub fn entity_into_model(entity: Article) -> ArticleModel {
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
            script: entity.get_script().map(ToString::to_string),
            cleanup_script: entity.get_cleanup_script().map(ToString::to_string),
        }
    }

    pub fn entity_into_active_model(entity: Article) -> ArticleActiveModel {
        let script_as_value = entity.get_script().map(ToString::to_string);
        let cleanup_script_as_value = entity.get_cleanup_script().map(ToString::to_string);

        ArticleActiveModel {
            id: entity.id().into_active_value(),
            author_id: entity.author_id().into_active_value(),
            cover_url: entity.cover_url().to_owned().into_active_value(),
            title: entity.title().to_owned().into_active_value(),
            content: entity.content().to_owned().into_active_value(),
            approved: entity.approved().into_active_value(),
            created_at: entity.created_at().into_active_value(),
            updated_at: entity.updated_at().into_active_value().into(),
            slug: entity.slug().to_string().into_active_value(),
            description: entity.description().to_owned().into_active_value(),
            script: if entity.has_been_touched() {
                Some(script_as_value).into_active_value()
            } else {
                script_as_value.into_active_value().into()
            },
            cleanup_script: if entity.has_been_touched() {
                Some(cleanup_script_as_value).into_active_value()
            } else {
                cleanup_script_as_value.into_active_value().into()
            },
        }
    }

    pub fn active_model_into_entity(
        active_model: ArticleActiveModel,
        tags: Vec<ArticleTag>,
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
            tags,
            active_model.script.unwrap(),
            active_model.cleanup_script.unwrap(),
        )
    }

    pub fn model_into_entity(model: ArticleModel, tags: Vec<ArticleTag>) -> Article {
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
            tags,
            model.script,
            model.cleanup_script,
        )
    }
}
