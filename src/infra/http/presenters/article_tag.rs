use serde::{Deserialize, Serialize};

use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize, Deserialize)]
pub struct MappedArticleTag {
    id: i32,
    value: String,
}

pub struct ArticleTagPresenter;

impl PresenterTrait<ArticleTag, MappedArticleTag> for ArticleTagPresenter {
    fn to_http(tag: ArticleTag) -> MappedArticleTag {
        MappedArticleTag {
            id: tag.id(),
            value: tag.value().into(),
        }
    }
}

impl ArticleTagPresenter {}
