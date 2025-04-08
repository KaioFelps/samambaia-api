use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::repositories::article_tag_repository::{
    ArticleTagQueryType,
    FindManyArticleTagsResponse,
    MockArticleTagRepositoryTrait,
};
use crate::error::SamambaiaError;

#[derive(Clone)]
pub struct ArticleTagArticle {
    pub article_id: Uuid,
    pub article_tag_id: i32,
}

pub fn get_article_tag_repository() -> (
    Arc<Mutex<Vec<ArticleTag>>>,
    Arc<Mutex<Vec<ArticleTagArticle>>>,
    MockArticleTagRepositoryTrait,
) {
    let db: Arc<Mutex<Vec<ArticleTag>>> = Arc::new(Mutex::new(Vec::new()));
    let article_tag_article_db = Arc::new(Mutex::new(Vec::<ArticleTagArticle>::new()));

    let mut repository = MockArticleTagRepositoryTrait::new();

    let db_clone = Arc::clone(&db);
    repository.expect_create().returning(move |draft_tag| {
        let id = db_clone.lock().unwrap().len() + 1;

        let tag = ArticleTag::new_from_existing(id as i32, draft_tag.value().into());
        db_clone.lock().unwrap().push(tag.clone());

        Ok(tag)
    });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_id().returning(move |id| {
        for tag in db_clone.lock().unwrap().iter() {
            if tag.id().eq(&id) {
                return Ok(Some(tag.clone()));
            }
        }

        Ok(None)
    });

    let db_clone = Arc::clone(&db);
    repository.expect_delete().returning(move |tag| {
        let mut new_db: Vec<ArticleTag> = vec![];

        for item in db_clone.lock().unwrap().iter() {
            if item.id().ne(&tag.id()) {
                new_db.push(item.clone());
            }
        }

        *db_clone.lock().unwrap() = new_db;
        Ok(())
    });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_value().returning(move |value| {
        for tag in db_clone.lock().unwrap().iter() {
            if tag.value().eq(&value) {
                return Ok(Some(tag.clone()));
            }
        }

        Ok(None)
    });

    let db_clone = Arc::clone(&db);
    repository.expect_save().returning(move |tag| {
        let mut index = None;

        for (i, item) in db_clone.lock().unwrap().iter().enumerate() {
            if item.id().eq(&tag.id()) {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(i) => {
                db_clone.lock().unwrap()[i] = tag.clone();
                Ok(tag)
            }
            None => Err(Box::new(SamambaiaError::resource_not_found_err())),
        }
    });

    let db_clone = Arc::clone(&db);
    repository.expect_find_many().returning(move |params| {
        let PaginationParameters {
            page,
            items_per_page,
            query,
        } = params;

        let mut tags: Vec<ArticleTag> = Vec::new();

        if query.is_some() {
            let ArticleTagQueryType::Value(value) = query.unwrap();

            for item in db_clone.lock().unwrap().iter() {
                if item
                    .value()
                    .to_lowercase()
                    .contains(&value.clone().to_lowercase()[..])
                {
                    tags.push(item.clone());
                }
            }
        } else {
            tags = db_clone.lock().unwrap().clone();
        }

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
    });

    let db_clone = db.clone();
    repository.expect_find_many_by_ids().returning(move |ids| {
        Ok(db_clone
            .lock()
            .unwrap()
            .iter()
            .filter(|tag| ids.iter().any(|id| tag.id().eq(id)))
            .cloned()
            .collect::<Vec<_>>())
    });

    let article_tag_article_db_clone = article_tag_article_db.clone();
    repository
        .expect_associate_tags_to_article()
        .returning(move |article, tags_ids| {
            let mut lock = article_tag_article_db_clone.lock().unwrap();
            tags_ids.iter().for_each(|tag_id| {
                lock.push(ArticleTagArticle {
                    article_id: article.id(),
                    article_tag_id: *tag_id,
                });
            });

            Ok(())
        });

    let article_tag_article_db_clone = article_tag_article_db.clone();
    repository
        .expect_disassociate_tags_from_article()
        .returning(move |article, tags_ids| {
            let new_db = article_tag_article_db_clone
                .lock()
                .unwrap()
                .iter()
                .filter(|tag| {
                    !(tag.article_id == article.id()
                        && tags_ids.iter().any(|tag_id| tag.article_tag_id.eq(tag_id)))
                })
                .cloned()
                .collect::<Vec<_>>();

            *article_tag_article_db_clone.lock().unwrap() = new_db;

            Ok(())
        });

    (db, article_tag_article_db, repository)
}
