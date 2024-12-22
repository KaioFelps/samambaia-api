use entities::article_tag::ActiveModel as ArticleTagActiveModel;
use entities::article_tag::Model as ArticleTagModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};

use super::SeaMapper;

pub struct SeaArticleTagMapper;

impl SeaMapper<ArticleTag, ArticleTagModel, ArticleTagActiveModel> for SeaArticleTagMapper {
    fn entity_into_model(entity: ArticleTag) -> ArticleTagModel {
        ArticleTagModel {
            id: entity.id(),
            value: entity.value().to_owned(),
        }
    }

    fn entity_into_active_model(entity: ArticleTag) -> ArticleTagActiveModel {
        ArticleTagActiveModel {
            id: entity.id().into_active_value(),
            value: entity.value().to_owned().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: ArticleTagActiveModel) -> ArticleTag {
        ArticleTag::new_from_existing(active_model.id.unwrap(), active_model.value.unwrap())
    }

    fn model_into_entity(model: ArticleTagModel) -> ArticleTag {
        ArticleTag::new_from_existing(model.id, model.value)
    }
}

impl SeaArticleTagMapper {
    pub fn draft_entity_into_active_model(draft_entity: DraftArticleTag) -> ArticleTagActiveModel {
        ArticleTagActiveModel {
            value: draft_entity.value().to_owned().into_active_value(),
            ..Default::default()
        }
    }
}
