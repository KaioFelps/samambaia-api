use std::error::Error;

use uuid::Uuid;

use super::LocalDb;
use crate::core::pagination::PaginationParameters;
use crate::domain::aggregations::article_preview::{ArticlePreview, ArticlePreviewAuthor};
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::slug::Slug;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::{
    ArticleQueryType,
    ArticleRepositoryTrait,
    FindManyArticlesPreviewsResponse,
    FindManyArticlesResponse,
};
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::error::SamambaiaError;

#[derive(Clone)]
pub struct InMemoryArticleRepository<ATR: ArticleTagRepositoryTrait + Send + Sync> {
    pub article_db: LocalDb<Article>,
    pub user_db: LocalDb<User>,
    article_tag_repository: ATR,
}

impl<ATR: ArticleTagRepositoryTrait + Send + Sync> InMemoryArticleRepository<ATR> {
    pub fn new(
        article_db: LocalDb<Article>,
        user_db: LocalDb<User>,
        article_tag_repository: ATR,
    ) -> Self {
        Self {
            article_db,
            user_db,
            article_tag_repository,
        }
    }

    pub fn default(article_tag_repository: ATR) -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            article_tag_repository,
        )
    }
}

#[async_trait::async_trait]
impl<ATR: ArticleTagRepositoryTrait + Send + Sync> ArticleRepositoryTrait
    for InMemoryArticleRepository<ATR>
{
    async fn create(&self, mut article: Article) -> Result<Article, Box<dyn Error>> {
        if let Some(changes) = article.get_tags_changeset() {
            if changes.has_changes() {
                self.article_tag_repository
                    .associate_tags_to_article(
                        article.id(),
                        changes.added.iter().map(|tag| tag.id()).collect(),
                    )
                    .await?;

                self.article_tag_repository
                    .disassociate_tags_from_article(
                        article.id(),
                        changes.removed.iter().map(|tag| tag.id()).collect(),
                    )
                    .await?;
            }
        }

        article.flush_tags();

        self.article_db.lock().unwrap().push(article.clone());
        Ok(article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>> {
        for article in self.article_db.lock().unwrap().iter() {
            if article.id().eq(&id) {
                return Ok(Some(article.clone()));
            }
        }

        Ok(None)
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, Box<dyn Error>> {
        Ok(self
            .article_db
            .lock()
            .unwrap()
            .iter()
            .find(|article| article.slug().eq(slug))
            .cloned())
    }

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesResponse, Box<dyn Error>> {
        let PaginationParameters {
            page,
            items_per_page,
            query,
        } = params;

        let mut articles: Vec<Article> = Vec::new();

        if query.is_some() {
            let query = query.unwrap();
            match query {
                ArticleQueryType::Title(content) => {
                    for item in self.article_db.lock().unwrap().iter() {
                        if item
                            .title()
                            .to_lowercase()
                            .contains(&content.clone().to_lowercase())
                        {
                            articles.push(item.clone());
                        }
                    }
                }
                ArticleQueryType::Author(content) => {
                    for item in self.article_db.lock().unwrap().iter() {
                        if item.author_id().eq(&content) {
                            articles.push(item.clone());
                        }
                    }
                }
                ArticleQueryType::Tag(tag_id) => {
                    for item in self.article_db.lock().unwrap().iter() {
                        if item.get_tags().iter().any(|tag| tag.id() == tag_id) {
                            articles.push(item.clone());
                        }
                    }
                }
            }
        } else {
            articles = self.article_db.lock().unwrap().clone();
        }

        if let Some(approved_filter) = show_only_approved_state {
            articles = articles
                .into_iter()
                .filter(|article| article.approved().eq(&approved_filter))
                .collect::<Vec<Article>>();
        }

        let total_of_items_before_paginating = articles.len();

        let leap = (page - 1) * items_per_page;

        let mut res_articles = vec![];

        for (index, item) in articles.into_iter().enumerate() {
            if index >= leap as usize {
                res_articles.push(item);
            }
        }

        Ok(FindManyArticlesResponse(
            res_articles,
            total_of_items_before_paginating as u64,
        ))
    }

    async fn find_many_previews(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesPreviewsResponse, Box<dyn Error>> {
        let mut articles = self.article_db.lock().unwrap().to_vec();
        let users = self.user_db.lock().unwrap().to_vec();

        articles.sort_by(|a, b| b.created_at().partial_cmp(&a.created_at()).unwrap());

        let mut parsed_articles = vec![];

        for article in articles {
            if let Some(approved) = show_only_approved_state {
                if article.approved() != approved {
                    continue;
                }
            }

            let author = match users.iter().find(|user| user.id().eq(&article.author_id())) {
                None => continue,
                Some(author) => author,
            };

            let author = ArticlePreviewAuthor::new(author.id(), author.nickname().to_owned());

            parsed_articles.push(ArticlePreview::new(
                article.id(),
                article.cover_url().to_owned(),
                article.title().to_owned(),
                article.description().to_owned(),
                article.approved(),
                article.get_tags().into_iter().cloned().collect(),
                author,
                article.created_at(),
                article.slug().to_owned(),
            ));
        }

        let count = parsed_articles.len() as u64;

        let offset = ((params.page - 1) * params.items_per_page) as usize;
        let articles = parsed_articles
            .into_iter()
            .skip(offset)
            .take(params.items_per_page as usize)
            .collect::<Vec<_>>();

        Ok(FindManyArticlesPreviewsResponse(articles, count))
    }

    async fn save(&self, mut article: Article) -> Result<Article, Box<dyn Error>> {
        let mut index = None;
        for (i, item) in self.article_db.lock().unwrap().iter().enumerate() {
            if item.id() == article.id() {
                index = Some(i);
                break;
            }
        }

        let index = match index {
            None => return Err(Box::new(SamambaiaError::resource_not_found_err())),
            Some(i) => i,
        };

        if let Some(changes) = article.get_tags_changeset() {
            if changes.has_changes() {
                self.article_tag_repository
                    .associate_tags_to_article(
                        article.id(),
                        changes.added.iter().map(|tag| tag.id()).collect(),
                    )
                    .await?;

                self.article_tag_repository
                    .disassociate_tags_from_article(
                        article.id(),
                        changes.removed.iter().map(|tag| tag.id()).collect(),
                    )
                    .await?;
            }
        }

        article.flush_tags();

        self.article_db.lock().unwrap()[index] = article.clone();
        Ok(article)
    }
}
