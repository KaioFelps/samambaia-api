use entities::article_tag::Model as ArticleTagModel;
use entities::article_tag::ActiveModel as ArticleTagActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};

pub struct SeaArticleTagMapper;

impl SeaArticleTagMapper {
    pub fn article_tag_to_sea_model(tag: ArticleTag) -> ArticleTagModel {
        return ArticleTagModel {
            id: tag.id(),
            value: tag.value().to_owned()
        };
    }

    pub fn draft_article_tag_to_sea_active_model(tag: DraftArticleTag) -> ArticleTagActiveModel {
        return ArticleTagActiveModel {
            value: tag.value().to_owned().into_active_value(),
            ..Default::default()
        };
    }

    pub fn active_model_to_article_tag(active_model_tag: ArticleTagActiveModel) -> ArticleTag {
        return ArticleTag::new_from_existing(
            active_model_tag.id.unwrap(),
            active_model_tag.value.unwrap()
        );
    }

    pub fn model_to_article_tag(model_tag: ArticleTagModel) -> ArticleTag {
        return ArticleTag::new_from_existing(
            model_tag.id,
            model_tag.value,
        );
    }
}