use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::{
    ArticleQueryType, FindManyArticlesResponse, MockArticleRepositoryTrait,
};
use crate::error::DomainError;
use std::sync::{Arc, Mutex};

pub fn get_article_repository() -> (Arc<Mutex<Vec<Article>>>, MockArticleRepositoryTrait) {
    let db: Arc<Mutex<Vec<Article>>> = Arc::new(Mutex::new(vec![]));
    let mut repository = MockArticleRepositoryTrait::new();

    let db_clone = Arc::clone(&db);
    repository
        .expect_create()
        .returning(move |article: Article| {
            db_clone.lock().unwrap().push(article.clone());
            Ok(article)
        });

    let db_clone = Arc::clone(&db);
    repository
        .expect_find_many()
        .returning(move |params, approved_status_filter| {
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
                        for item in db_clone.lock().unwrap().iter() {
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
                        for item in db_clone.lock().unwrap().iter() {
                            if item.author_id().eq(&content) {
                                articles.push(item.clone());
                            }
                        }
                    }
                    ArticleQueryType::Tag(tag_id) => {
                        for item in db_clone.lock().unwrap().iter() {
                            if item.tag_id().unwrap().eq(&tag_id) {
                                articles.push(item.clone());
                            }
                        }
                    }
                }
            } else {
                articles = db_clone.lock().unwrap().clone();
            }

            if approved_status_filter.is_some() {
                let approved_filter: bool = approved_status_filter.unwrap();
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
        });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_id().returning(move |id| {
        for article in db_clone.lock().unwrap().iter() {
            if article.id().eq(&id) {
                return Ok(Some(article.clone()));
            }
        }

        Ok(None)
    });

    let db_clone = Arc::clone(&db);
    repository
        .expect_save()
        .returning(move |param_article: Article| {
            let mut index = None;
            for (i, item) in db_clone.lock().unwrap().iter().enumerate() {
                if item.id() == param_article.id() {
                    index = Some(i);
                    break;
                }
            }

            match index {
                None => Err(Box::new(DomainError::resource_not_found_err())),
                Some(i) => {
                    db_clone.lock().unwrap()[i] = param_article.clone();
                    Ok(param_article)
                }
            }
        });

    let db_clone = Arc::clone(&db);
    repository
        .expect_find_by_slug()
        .returning(move |article_slug| {
            let mut article: Option<Article> = None;

            for item in db_clone.lock().unwrap().iter() {
                if item.slug().eq(article_slug) {
                    article = Some(item.clone());
                    break;
                }
            }

            Ok(article)
        });

    let db_clone = Arc::clone(&db);
    repository.expect_get_home_articles().returning(move || {
        let mut articles = db_clone.lock().unwrap().clone();
        articles.sort_by(|a, b| b.created_at().partial_cmp(&a.created_at()).unwrap());

        let articles = &articles[0..=2];
        let articles = articles.iter().map(|article| article.to_owned()).collect();

        Ok(articles)
    });

    (db, repository)
}
