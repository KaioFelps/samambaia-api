use std::sync::{Arc, Mutex};
use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::repositories::article_tag_repository::{ArticleTagQueryType, FindManyArticleTagsResponse, MockArticleTagRepositoryTrait};
use crate::errors::resource_not_found::ResourceNotFoundError;

pub fn get_article_tag_repository() -> (Arc<Mutex<Vec<ArticleTag>>>, MockArticleTagRepositoryTrait) {
    let db: Arc<Mutex<Vec<ArticleTag>>>= Arc::new(Mutex::new(Vec::new()));

    let mut repository = MockArticleTagRepositoryTrait::new();

    let db_clone = Arc::clone(&db);
    repository.expect_create()
        .returning(move |draft_tag| {
            let id = db_clone.lock().unwrap().len() + 1;

            let tag = ArticleTag::new_from_existing(id as i32, draft_tag.value().into());
            db_clone.lock().unwrap().push(tag.clone());

            Ok(tag)
        });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_id()
        .returning(move |id| {
            for tag in db_clone.lock().unwrap().iter() {
                if tag.id().eq(&id) {
                    return Ok(Some(tag.clone()));
                }
            }

            return Ok(None);
        });

    let db_clone = Arc::clone(&db);
    repository.expect_delete()
        .returning(move |tag| {
            let mut new_db: Vec<ArticleTag> = vec![];

            for item in db_clone.lock().unwrap().iter() {
                if item.id().eq(&tag.id()) {
                    new_db.push(item.clone());
                }
            }

            *db_clone.lock().unwrap() = new_db;
            Ok(())
        });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_value()
        .returning(move |value| {
            for tag in db_clone.lock().unwrap().iter() {
                if tag.value().eq(&value) {
                    return Ok(Some(tag.clone()));
                }
            }

            return Ok(None);
        });

    let db_clone = Arc::clone(&db);
    repository.expect_save()
        .returning(move |tag| {
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
                },
                None => Err(Box::new(ResourceNotFoundError::new()))
            }
        });

    let db_clone = Arc::clone(&db);
    repository.expect_find_many()
        .returning(move |params| {
            let PaginationParameters { page, items_per_page, query } = params;

            let mut tags: Vec<ArticleTag> = Vec::new();

            if query.is_some() {
                let ArticleTagQueryType::Value(value) = query.unwrap();

                for item in db_clone.lock().unwrap().iter() {
                    if item.value().to_lowercase().contains(&value.clone().to_lowercase()[..]) {
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

            Ok(FindManyArticleTagsResponse (res_tags, total_of_items_before_paginating as u64))
        });

    (db, repository)
}
