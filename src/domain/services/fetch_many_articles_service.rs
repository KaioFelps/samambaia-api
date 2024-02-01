use std::error::Error;

use crate::core::pagination::{PaginationParameters, PaginationResponse, Query, QueryType};
use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::{ArticleRepositoryTrait, FindManyResponse};
use crate::errors::internal_error::InternalError;

pub struct FetchManyArticlesParams {
    page: Option<u32>,
    per_page: Option<u32>,
    query: Option<String>,
    query_by: Option<QueryType>
}

pub struct FetchManyArticlesService<ArticleRepository : ArticleRepositoryTrait> {
    article_repository: ArticleRepository,
}

#[derive(Debug)]
pub struct FetchManyArticlesResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<Article>
}

impl<ArticleRepository: ArticleRepositoryTrait> FetchManyArticlesService<ArticleRepository> {
    pub fn new(article_repository: ArticleRepository) -> Self {
        FetchManyArticlesService {
            article_repository
        }
    }

    pub async fn exec(&self, params: FetchManyArticlesParams) -> Result<FetchManyArticlesResponse, Box<dyn Error>> {
        let default_items_per_page = 9;
        let default_page = 1;

        let items_per_page = if params.per_page.is_some() { params.per_page.unwrap() } else { default_items_per_page };
        let page = if params.page.is_some() { params.page.unwrap() } else { default_page };
        let query = if params.query.is_some() {
            Some(
                Query {
                    content: params.query.unwrap(),
                    query_type: params.query_by.unwrap(),
                }
            )
        } else {
            None
        };

        let response = self.article_repository.find_many(PaginationParameters {
            items_per_page,
            page,
            query
        }).await;

        if response.is_err() {
            dbg!(response.unwrap_err());
            return Err(Box::new(InternalError::new()));
        }

        let response = response.unwrap();
        let FindManyResponse (articles, total_items) = response;

        Ok(FetchManyArticlesResponse {
            data: articles,
            pagination: PaginationResponse {
                current_page: page,
                total_items,
                total_pages: (total_items as f64 / items_per_page as f64).ceil() as u32
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio;
    use uuid::Uuid;

    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;

    #[tokio::test]
    async fn test() {
        let mut db: Vec<Article> = Vec::new();

        let user = User::new("Floricultor".to_string(), "password".to_string(), Some(Role::Principal));

        db.push(Article::new(user.id(), "Article 1 title".to_string(), "Article 1 content here".to_string(), "url".to_string()));
        db.push(Article::new(user.id(), "Article 2 title".to_string(), "Article 2 content here".to_string(), "url".to_string()));

        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        mocked_article_repo
        .expect_find_many()
        .returning(move |params| {
            let PaginationParameters { page, items_per_page, query } = params;

            let mut articles: Vec<Article> = Vec::new();

            if query.is_some() {
                let content = query.as_ref().unwrap().content.clone();
                let by = query.unwrap().query_type;
                
                for item in db.iter() {
                    match by {
                        QueryType::TITLE => {
                            if item.title().to_lowercase().contains(&content.to_lowercase()[..]) {
                                articles.push(item.clone());
                            }
                        },
                        QueryType::AUTHOR => {
                            let user = user.clone();

                            let is_user = content == user.nickname();

                            #[warn(unused_mut)]
                            let mut id: Uuid;
                            id = Uuid::new_v4();

                            if is_user {
                                id = user.id();
                            }
                            
                            if item.author_id().eq(&id) {
                                articles.push(item.clone());
                            }
                        }
                    }
                }
            } else {
                articles = db.clone();
            }

            let total_of_items_before_paginating = articles.len();

            let leap = (page - 1) * items_per_page;

            /* SAMPLE
            * page = 2
            * items per page = 9
            * leap = (2 - 1)*9 = 9
            * articles from index 8 (leap - 1) on
            */

            let mut res_articles = vec![];

            for (index, item) in articles.iter().enumerate() {
                if index >= leap as usize {
                    res_articles.push(item.to_owned());
                }
            }

            Ok(FindManyResponse (res_articles, total_of_items_before_paginating as u64))
        });

        let fetch_many_articles_service = FetchManyArticlesService::new(mocked_article_repo);

        // make a request querying by title
        let res = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: Some(2),
            per_page: Some(1),
            query: Some("article".to_string()),
            query_by: Some(QueryType::TITLE)
        }).await.unwrap();

        assert_eq!(1, res.data.len());
        assert_eq!(res.pagination, PaginationResponse { current_page: 2, total_pages: 2, total_items: 2 });
        assert_eq!(res.data[0].title(), "Article 2 title");

        // make a request with no query
        let res_2 = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: None,
            per_page: None,
            query: None,
            query_by: None,
        }).await.unwrap();

        assert_eq!(2, res_2.data.len());
        assert_eq!(res_2.pagination, PaginationResponse { current_page: 1, total_pages: 1, total_items: 2 });
        assert_eq!(res_2.data[0].title(), "Article 1 title");
        assert_eq!(res_2.data[1].title(), "Article 2 title");

        // make a request querying by nickname that does not exist
        let res_3 = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: None,
            per_page: None,
            query: Some("Vamp".to_string()),
            query_by: Some(QueryType::AUTHOR),
        }).await.unwrap();

        assert_eq!(0, res_3.data.len());
        assert_eq!(res_3.pagination, PaginationResponse { current_page: 1, total_pages: 0, total_items: 0 });

        // make a request querying by nickname that exists
        let res_4 = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: None,
            per_page: None,
            query: Some("Floricultor".to_string()),
            query_by: Some(QueryType::AUTHOR),
        }).await.unwrap();

        assert_eq!(2, res_4.data.len());
        assert_eq!(res_4.pagination, PaginationResponse { current_page: 1, total_pages: 1, total_items: 2 });

        /* RESPONSE SAMPLE

        FetchManyArticlesResponse {
            pagination: PaginationResponse {
                current_page: 2,
                total_pages: 2,
                total_items: 2,
            },
            data: [
                Article {
                    id: 2f5627eb-fb84-4713-8ca7-6a46d48f17d9,
                    author_id: 1329fda2-4c14-4fe6-b4dc-6f698ef24f7e,
                    cover_url: \"url\",
                    title: \"Article 2 title\",
                    content: \"Article 2 content here\",
                    approved: false,
                    created_at: 2024-02-01T05:25:05.670068700,
                    updated_at: None,
                },
            ],
        }

        */
    }
}