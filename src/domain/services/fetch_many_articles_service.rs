use log::error;

use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::{ArticleQueryType, ArticleRepositoryTrait, FindManyResponse};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;

use crate::{LOG_SEP, R_EOL};

type Error = Box<dyn DomainErrorTrait>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceArticleQueryType {
    Title(String),
    Author(String),
}

pub struct FetchManyArticlesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<ServiceArticleQueryType>
}

pub struct FetchManyArticlesService<ArticleRepository, UserRepository>
where ArticleRepository: ArticleRepositoryTrait, UserRepository: UserRepositoryTrait
{
    article_repository: Box<ArticleRepository>,
    user_repository: Box<UserRepository>,
}

#[derive(Debug)]
pub struct FetchManyArticlesResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<Article>
}

impl<ArticleRepository: ArticleRepositoryTrait, UserRepository: UserRepositoryTrait>
FetchManyArticlesService<ArticleRepository, UserRepository> {
    // CONSTRUCTOR
    pub fn new(article_repository: Box<ArticleRepository>, user_repository: Box<UserRepository>) -> Self {
        FetchManyArticlesService {
            article_repository,
            user_repository
        }
    }

    pub async fn exec(&self, params: FetchManyArticlesParams) -> Result<FetchManyArticlesResponse, Error> {
        let default_items_per_page = 9;
        let default_page = 1;

        let items_per_page = if params.per_page.is_some() { params.per_page.unwrap() } else { default_items_per_page };
        
        let page = if params.page.is_some() {
            let params_page = params.page.unwrap();
            if params_page <= 0 { default_page } else { params_page }
        } else { default_page };
        
        let query = self.parse_query(params.query).await;

        if let Err(err) = query {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Articles Service, while parsing the query: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                err.as_ref()
            );
            
            return Err(err)
        }

        let query = query.unwrap();

        let response = self.article_repository.find_many(PaginationParameters {
            items_per_page,
            page,
            query
        }).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Articles Service, while finding many articles from database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

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
    
    async fn parse_query(&self, query: Option<ServiceArticleQueryType>) -> Result<Option<ArticleQueryType>, Error> {
        if query.is_none() {
            return Ok(None);
        }

        match query.unwrap() {
            ServiceArticleQueryType::Author(content) => {
                let user = self.user_repository.find_by_nickname(&content).await;

                if user.is_err() {
                    return Err(Box::new(InternalError::new()));
                }

                let user = user.unwrap();

                if user.is_none() {
                    return Err(Box::new(ResourceNotFoundError::new()));
                }

                Ok(Some(ArticleQueryType::Author(user.unwrap().id())))
            },
            ServiceArticleQueryType::Title(content) => {
                Ok(Some(ArticleQueryType::Title(content)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use http::StatusCode;
    use tokio;

    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;

    #[tokio::test]
    async fn test() {
        let mut db: Vec<Article> = Vec::new();

        let user = User::new("Floricultor".to_string(), "password".to_string(), Some(Role::Principal));

        db.push(Article::new(user.id(), "Article 1 title".to_string(), "Article 1 content here".to_string(), "url".to_string()));
        db.push(Article::new(user.id(), "Article 2 title".to_string(), "Article 2 content here".to_string(), "url".to_string()));

        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();

        mocked_user_repo
        .expect_find_by_nickname()
        .returning(move |nickname| {
            let user = user.clone();

            let is_user = nickname == user.nickname();

            if is_user {
                return Ok(Some(user));
            }

            Ok(None)
        });

        mocked_article_repo
        .expect_find_many()
        .returning(move |params| {
            let PaginationParameters { page, items_per_page, query } = params;

            let mut articles: Vec<Article> = Vec::new();

            if query.is_some() {
                let query = query.unwrap();
                    match query {
                        ArticleQueryType::Title(content) => {
                            for item in db.iter() {
                                if item.title().to_lowercase().contains(&content.clone().to_lowercase()[..]) {
                                    articles.push(item.clone());
                                }
                            }
                        },
                        ArticleQueryType::Author(content) => {
                            for item in db.iter() {
                                if item.author_id().eq(&content) {
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

        let fetch_many_articles_service = FetchManyArticlesService::new(Box::new(mocked_article_repo), Box::new(mocked_user_repo));

        // make a request querying by title
        let res = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: Some(2),
            per_page: Some(1),
            query: Some(ServiceArticleQueryType::Title("article".to_string()))
        }).await.unwrap();

        assert_eq!(1, res.data.len());
        assert_eq!(res.pagination, PaginationResponse { current_page: 2, total_pages: 2, total_items: 2 });
        assert_eq!(res.data[0].title(), "Article 2 title");

        // make a request with no query
        let res_2 = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: None,
            per_page: None,
            query: None,
        }).await.unwrap();

        assert_eq!(2, res_2.data.len());
        assert_eq!(res_2.pagination, PaginationResponse { current_page: 1, total_pages: 1, total_items: 2 });
        assert_eq!(res_2.data[0].title(), "Article 1 title");
        assert_eq!(res_2.data[1].title(), "Article 2 title");

        // make a request querying by nickname that does not exist
        let res_3 = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: None,
            per_page: None,
            query: Some(ServiceArticleQueryType::Author("Vamp".to_string())),
        }).await.unwrap_err();

        assert_eq!(res_3.code(), &StatusCode::NOT_FOUND);

        // make a request querying by nickname that exists
        let res_4 = fetch_many_articles_service.exec(FetchManyArticlesParams {
            page: None,
            per_page: None,
            query: Some(ServiceArticleQueryType::Author("Floricultor".to_string())),
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