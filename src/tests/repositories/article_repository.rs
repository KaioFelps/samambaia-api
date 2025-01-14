use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::{
    ArticleQueryType, FindManyArticlesResponse, MockArticleRepositoryTrait,
};
use crate::error::DomainError;
use crate::infra::http::presenters::home_article::HomeArticlePresenter;
use crate::infra::http::presenters::presenter::PresenterTrait;
use std::sync::{Arc, Mutex};

type LocalDb<T> = Arc<Mutex<Vec<T>>>;

pub fn get_article_repository() -> (LocalDb<Article>, LocalDb<User>, MockArticleRepositoryTrait) {
    let articles_db: Arc<Mutex<Vec<Article>>> = Arc::new(Mutex::new(vec![]));
    let users_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![]));
    let mut repository = MockArticleRepositoryTrait::new();

    let articles_db_clone = Arc::clone(&articles_db);
    repository
        .expect_create()
        .returning(move |article: Article| {
            articles_db_clone.lock().unwrap().push(article.clone());
            Ok(article)
        });

    let articles_db_clone = Arc::clone(&articles_db);
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
                        for item in articles_db_clone.lock().unwrap().iter() {
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
                        for item in articles_db_clone.lock().unwrap().iter() {
                            if item.author_id().eq(&content) {
                                articles.push(item.clone());
                            }
                        }
                    }
                    ArticleQueryType::Tag(tag_id) => {
                        for item in articles_db_clone.lock().unwrap().iter() {
                            if item.tag_id().unwrap().eq(&tag_id) {
                                articles.push(item.clone());
                            }
                        }
                    }
                }
            } else {
                articles = articles_db_clone.lock().unwrap().clone();
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

    let articles_db_clone = Arc::clone(&articles_db);
    repository.expect_find_by_id().returning(move |id| {
        for article in articles_db_clone.lock().unwrap().iter() {
            if article.id().eq(&id) {
                return Ok(Some(article.clone()));
            }
        }

        Ok(None)
    });

    let articles_db_clone = Arc::clone(&articles_db);
    repository
        .expect_save()
        .returning(move |param_article: Article| {
            let mut index = None;
            for (i, item) in articles_db_clone.lock().unwrap().iter().enumerate() {
                if item.id() == param_article.id() {
                    index = Some(i);
                    break;
                }
            }

            match index {
                None => Err(Box::new(DomainError::resource_not_found_err())),
                Some(i) => {
                    articles_db_clone.lock().unwrap()[i] = param_article.clone();
                    Ok(param_article)
                }
            }
        });

    let articles_db_clone = Arc::clone(&articles_db);
    repository
        .expect_find_by_slug()
        .returning(move |article_slug| {
            let mut article: Option<Article> = None;

            for item in articles_db_clone.lock().unwrap().iter() {
                if item.slug().eq(article_slug) {
                    article = Some(item.clone());
                    break;
                }
            }

            Ok(article)
        });

    let articles_db_clone = Arc::clone(&articles_db);
    let users_db_clone = Arc::clone(&users_db);
    repository.expect_get_home_articles().returning(move || {
        let mut articles = articles_db_clone.lock().unwrap().clone();
        let users = users_db_clone.lock().unwrap().clone();

        articles.sort_by(|a, b| b.created_at().partial_cmp(&a.created_at()).unwrap());

        let articles = articles
            .iter()
            .take(6)
            .map(|article| {
                let user = users.iter().find(|user| user.id().eq(&article.author_id()));
                (article, user)
            })
            .collect::<Vec<_>>();

        let mut mapped_articles = vec![];

        for (article, user) in articles {
            match user {
                None => {
                    println!("Encountered an article that has no author: {:#?}", article);
                    return Err(Box::new(DomainError::internal_err()));
                }
                Some(user) => {
                    mapped_articles.push(HomeArticlePresenter::to_http((
                        article.clone(),
                        user.clone(),
                    )));
                }
            }
        }

        Ok(mapped_articles)
    });

    (articles_db, users_db, repository)
}
