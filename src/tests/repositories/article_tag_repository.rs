use std::error::Error;

use async_trait::async_trait;
use uuid::Uuid;

use super::{get_local_db, LocalDb};
use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
use crate::domain::repositories::article_tag_repository::{
    ArticleTagQueryType,
    ArticleTagRepositoryTrait,
    FindManyArticleTagsResponse,
};
use crate::error::SamambaiaError;

#[derive(Clone)]
pub struct InMemoryArticleTagRepository {
    pub tag_db: LocalDb<ArticleTag>,
    pub article_tag_db: LocalDb<ArticleTagArticle>,
}

#[derive(Clone)]
pub struct ArticleTagArticle {
    pub article_id: Uuid,
    pub article_tag_id: i32,
}

impl InMemoryArticleTagRepository {
    #[allow(dead_code)]
    pub fn new(tag_db: LocalDb<ArticleTag>, article_tag_db: LocalDb<ArticleTagArticle>) -> Self {
        Self {
            article_tag_db,
            tag_db,
        }
    }
}

impl Default for InMemoryArticleTagRepository {
    fn default() -> Self {
        Self {
            article_tag_db: get_local_db(),
            tag_db: get_local_db(),
        }
    }
}

#[async_trait]
impl ArticleTagRepositoryTrait for InMemoryArticleTagRepository {
    async fn create(&self, draft_tag: DraftArticleTag) -> Result<ArticleTag, Box<dyn Error>> {
        let id = self.tag_db.lock().unwrap().len() + 1;

        let tag = ArticleTag::new_from_existing(id as i32, draft_tag.value().into());
        self.tag_db.lock().unwrap().push(tag.clone());

        Ok(tag)
    }

    async fn find_by_id(&self, tag_id: i32) -> Result<Option<ArticleTag>, Box<dyn Error>> {
        Ok(self
            .tag_db
            .lock()
            .unwrap()
            .iter()
            .find(|tag| tag.id() == tag_id)
            .cloned())
    }

    async fn find_by_value(&self, tag_value: String) -> Result<Option<ArticleTag>, Box<dyn Error>> {
        Ok(self
            .tag_db
            .lock()
            .unwrap()
            .iter()
            .find(|tag| tag.value() == &tag_value)
            .cloned())
    }

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleTagQueryType>,
    ) -> Result<FindManyArticleTagsResponse, Box<dyn Error>> {
        let PaginationParameters {
            page,
            items_per_page,
            query,
        } = params;

        let tags = match query {
            None => self.tag_db.lock().unwrap().to_vec(),
            Some(tag) => match tag {
                ArticleTagQueryType::Value(value) => self
                    .tag_db
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|tag| {
                        tag.value()
                            .to_lowercase()
                            .contains(&value.clone().to_lowercase()[..])
                    })
                    .cloned()
                    .collect::<Vec<_>>(),
            },
        };

        let total_of_items_before_paginating = tags.len();

        let leap = (page - 1) * items_per_page;

        let mut res_tags = vec![];

        for (index, item) in tags.iter().enumerate() {
            if index >= leap as usize {
                res_tags.push(item.to_owned());
            }
        }

        Ok(FindManyArticleTagsResponse(
            res_tags,
            total_of_items_before_paginating as u64,
        ))
    }

    async fn save(&self, article_tag: ArticleTag) -> Result<ArticleTag, Box<dyn Error>> {
        let index = match self
            .tag_db
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, tag)| tag.id() == article_tag.id())
            .map(|(index, _)| index)
        {
            None => return Err(Box::new(SamambaiaError::resource_not_found_err())),
            Some(index) => index,
        };

        self.tag_db.lock().unwrap()[index] = article_tag.clone();
        Ok(article_tag)
    }

    async fn delete(&self, article_tag: ArticleTag) -> Result<(), Box<dyn Error>> {
        let new_db: Vec<ArticleTag> = self
            .tag_db
            .lock()
            .unwrap()
            .iter()
            .filter(|tag| tag.id() != article_tag.id())
            .cloned()
            .collect::<Vec<_>>();

        *self.tag_db.lock().unwrap() = new_db;
        Ok(())
    }

    async fn find_many_by_ids(&self, tag_ids: Vec<i32>) -> Result<Vec<ArticleTag>, SamambaiaError> {
        Ok(self
            .tag_db
            .lock()
            .unwrap()
            .iter()
            .filter(|tag| tag_ids.iter().any(|id| tag.id().eq(id)))
            .cloned()
            .collect::<Vec<_>>())
    }

    async fn associate_tags_to_article(
        &self,
        article_id: Uuid,
        tags_ids: Vec<i32>,
    ) -> Result<(), SamambaiaError> {
        let mut lock = self.article_tag_db.lock().unwrap();
        tags_ids.iter().for_each(|tag_id| {
            lock.push(ArticleTagArticle {
                article_id,
                article_tag_id: *tag_id,
            });
        });

        Ok(())
    }

    async fn disassociate_tags_from_article(
        &self,
        article_id: Uuid,
        tags_ids: Vec<i32>,
    ) -> Result<(), SamambaiaError> {
        let new_db = self
            .article_tag_db
            .lock()
            .unwrap()
            .iter()
            .filter(|tag| {
                !(tag.article_id == article_id
                    && tags_ids.iter().any(|tag_id| tag.article_tag_id.eq(tag_id)))
            })
            .cloned()
            .collect::<Vec<_>>();

        *self.article_tag_db.lock().unwrap() = new_db;

        Ok(())
    }
}
