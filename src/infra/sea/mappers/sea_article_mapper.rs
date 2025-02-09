use entities::article::{ActiveModel as ArticleActiveModel, Model as ArticleModel};
use sea_orm::IntoActiveValue;

use super::SeaMapper;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::slug::Slug;

pub struct SeaArticleMapper;

impl SeaMapper<Article, ArticleModel, ArticleActiveModel> for SeaArticleMapper {
    fn entity_into_model(entity: Article) -> ArticleModel {
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
            tag_id: entity.tag_id(),
            tag_value: entity.tag_value().clone(),
            description: entity.description().to_string(),
        }
    }

    fn entity_into_active_model(entity: Article) -> ArticleActiveModel {
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
            tag_value: entity.tag_value().to_owned().into_active_value(),
            tag_id: entity.tag_id().into_active_value(),
            description: entity.description().to_owned().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: ArticleActiveModel) -> Article {
        Article::new_from_existing(
            active_model.id.unwrap(),
            active_model.author_id.unwrap(),
            active_model.cover_url.unwrap(),
            active_model.title.unwrap(),
            active_model.content.unwrap(),
            active_model.approved.unwrap(),
            active_model.created_at.unwrap(),
            active_model.updated_at.unwrap(),
            active_model.tag_id.unwrap(),
            active_model.tag_value.unwrap(),
            Slug::new_from_existing(active_model.slug.unwrap()),
            active_model.description.unwrap(),
        )
    }

    fn model_into_entity(model: ArticleModel) -> Article {
        Article::new_from_existing(
            model.id,
            model.author_id,
            model.cover_url,
            model.title,
            model.content,
            model.approved,
            model.created_at,
            model.updated_at,
            model.tag_id,
            model.tag_value,
            Slug::new_from_existing(model.slug),
            model.description,
        )
    }
}
