use entities::article::ActiveModel as ArticleActiveModel;
use entities::article::Model as ArticleModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::slug::Slug;

pub struct SeaArticleMapper {}

impl SeaArticleMapper {
    pub fn article_to_sea_model(article: Article) -> ArticleModel {
        let sea_model = ArticleModel {
            id: article.id(),
            author_id: article.author_id(),
            cover_url: article.cover_url().to_owned(),
            title: article.title().to_owned(),
            content: article.content().to_owned(),
            approved: article.approved(),
            created_at: article.created_at(),
            updated_at: article.updated_at(),
            slug: article.slug().to_string(),
            tag_id: article.tag_id(),
            tag_value: article.tag_value(),
        };

        sea_model
    }

    pub fn article_to_sea_active_model(article: Article) -> ArticleActiveModel {
        let sea_active_model = ArticleActiveModel {
            id: article.id().into_active_value(),
            author_id: article.author_id().into_active_value(),
            cover_url: article.cover_url().to_owned().into_active_value(),
            title: article.title().to_owned().into_active_value(),
            content: article.content().to_owned().into_active_value(),
            approved: article.approved().into_active_value(),
            created_at: article.created_at().into_active_value(),
            updated_at: article.updated_at().into_active_value(),
            slug: article.slug().to_string().into_active_value(),
            tag_value: article.tag_value().into_active_value(),
            tag_id: article.tag_id().into_active_value(),
        };

        sea_active_model
    }

    pub fn active_model_to_article(active_model_article: ArticleActiveModel) -> Article {
        Article::new_from_existing(
            active_model_article.id.unwrap(),
            active_model_article.author_id.unwrap(),
            active_model_article.cover_url.unwrap(),
            active_model_article.title.unwrap(),
            active_model_article.content.unwrap(),
            active_model_article.approved.unwrap(),
            active_model_article.created_at.unwrap(),
            active_model_article.updated_at.unwrap(),
            active_model_article.tag_id.unwrap(),
            active_model_article.tag_value.unwrap(),
            Slug::new_from_existing(active_model_article.slug.unwrap()),
        )
    }

    pub fn model_to_article(model_article: ArticleModel) -> Article {
        Article::new_from_existing(
            model_article.id,
            model_article.author_id,
            model_article.cover_url,
            model_article.title,
            model_article.content,
            model_article.approved,
            model_article.created_at,
            model_article.updated_at,
            model_article.tag_id,
            model_article.tag_value,
            Slug::new_from_existing(model_article.slug),
        )
    }
}
