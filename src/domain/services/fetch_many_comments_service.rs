use log::error;
use uuid::Uuid;

use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_comment_repository::{ArticleCommentRepositoryTrait, CommentQueryType, FindManyCommentsResponse};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;

use crate::{LOG_SEP, R_EOL};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceCommentQueryType {
    Author(String),
    Content(String),
}

pub struct FetchManyCommentsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<ServiceCommentQueryType>
}

pub struct FetchManyCommentsService<ArticleCommentRepository, UserRepository>
where ArticleCommentRepository: ArticleCommentRepositoryTrait, UserRepository: UserRepositoryTrait
{
    article_comment_repository: Box<ArticleCommentRepository>,
    user_repository: Box<UserRepository>,
}

#[derive(Debug)]
pub struct FetchManyCommentsResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<Comment>
}

type ExecFuncReturn = Result<FetchManyCommentsResponse, Box<dyn DomainErrorTrait>>;

impl<ArticleCommentRepository: ArticleCommentRepositoryTrait, UserRepository: UserRepositoryTrait>
FetchManyCommentsService<ArticleCommentRepository, UserRepository> {
    // CONSTRUCTOR
    pub fn new(article_comment_repository: Box<ArticleCommentRepository>, user_repository: Box<UserRepository>) -> Self {
        FetchManyCommentsService {
            article_comment_repository,
            user_repository
        }
    }

    pub async fn exec_with_article_id(&self, article_id: Uuid, include_inactive: bool, params: FetchManyCommentsParams) -> ExecFuncReturn {
        self.fetch(Some(article_id), include_inactive, params).await
    }

    pub async fn exec(&self, include_inactive: bool, params: FetchManyCommentsParams) -> ExecFuncReturn {
        self.fetch(None, include_inactive, params).await
    }

    async fn fetch(&self, article_id: Option<Uuid>, include_inactive: bool, params: FetchManyCommentsParams) -> ExecFuncReturn {
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
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Comments Service, while parsing the query: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                err.as_ref()
            );

            return Err(err)
        }

        let query = query.unwrap();

        let response = self.article_comment_repository.
            find_many_comments(article_id, include_inactive, PaginationParameters {
                items_per_page,
                page,
                query
            }).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Comments Service, while fetching many comments from database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let response = response.unwrap();
        let FindManyCommentsResponse (comments, total_items) = response;

        Ok(FetchManyCommentsResponse {
            data: comments,
            pagination: PaginationResponse {
                current_page: page,
                total_items,
                total_pages: (total_items as f64 / items_per_page as f64).ceil() as u32
            }
        })
    }

    async fn parse_query(&self, query: Option<ServiceCommentQueryType>) -> Result<Option<CommentQueryType>, Box<dyn DomainErrorTrait>> {
        if query.is_none() {
            return Ok(None);
        }

        let query = query.unwrap();

        match query {
            ServiceCommentQueryType::Author(content) => {
                let user = self.user_repository.find_by_nickname(&content).await;

                if user.is_err() {
                    return Err(Box::new(InternalError::new()));
                }

                let user = user.unwrap();

                if user.is_none() {
                    return Err(Box::new(ResourceNotFoundError::new()));
                }

                let content = user.unwrap().id();

                Ok(Some(CommentQueryType::Author(content)))
            },
            ServiceCommentQueryType::Content(content) => {
                Ok(Some(CommentQueryType::Content(content)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use http::StatusCode;
    use tokio;

    use crate::domain::domain_entities::article::Article;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::repositories::article_comment_repository::MockArticleCommentRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::libs::time::TimeHelper;

    #[tokio::test]
    async fn test() {
        let mut db: Vec<Comment> = Vec::new();

        let user = User::new("Floricultor".to_string(), "password".to_string(), Some(Role::Principal));
        let article = Article::new(user.id(), "Título da notícia".into(), "Conteúdo da notícia".into(), "url do cover".into());

        db.push(Comment::new(user.id(), Some(article.id()), "Comment 1 content here".to_string()));
        db.push(Comment::new(user.id(), Some(article.id()), "Comment 2 content here".to_string()));
        db.push(Comment::new_from_existing(
            Uuid::new_v4(),
            Some(article.id()),
            user.id(),
            "Coment 2 content here".into(),
            false,
            TimeHelper::now()
        ));

        let mut mocked_comment_repo: MockArticleCommentRepositoryTrait = MockArticleCommentRepositoryTrait::new();
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

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
        .expect_find_by_id()
        .returning(move |article_id| {
            let is_article = article.id().eq(&article_id);

            if is_article {
                return Ok(Some(article.clone()));
            }
            
            Ok(None)
        });

        mocked_comment_repo
        .expect_find_many_comments()
        .returning(move |_article_id, include_inactive, params| {
            let PaginationParameters { page, items_per_page, query } = params;

            let mut comments: Vec<Comment> = Vec::new();

            if query.is_some() {                
                match query.unwrap() {
                    CommentQueryType::Content(content) => {
                        for item in db.iter() {
                            if
                                item.content().to_lowercase().contains(&content.to_lowercase()[..])
                                && (include_inactive || item.is_active())
                            {
                                comments.push(item.clone());
                            }
                        }
                    },
                    CommentQueryType::Author(content) => {
                        for item in db.iter() {
                            if
                                item.author_id().eq(&content)
                                && (include_inactive || item.is_active())
                            {
                                comments.push(item.clone());
                            }
                        }
                    }
                }
            } else {
                for item in db.iter() {
                    if include_inactive || item.is_active() {
                        comments.push(item.clone());
                    }
                }
            }

            let total_of_items_before_paginating = comments.len();

            let leap = (page - 1) * items_per_page;

            /* SAMPLE
            * page = 2
            * items per page = 9
            * leap = (2 - 1)*9 = 9
            * comments from index 8 (leap - 1) on
            */

            let mut res_comments = vec![];

            for (index, item) in comments.iter().enumerate() {
                if index >= leap as usize {
                    res_comments.push(item.to_owned());
                }
            }

            Ok(FindManyCommentsResponse (res_comments, total_of_items_before_paginating as u64))
        });

        let fetch_many_comments_service = FetchManyCommentsService::new(Box::new(mocked_comment_repo), Box::new(mocked_user_repo));

        // make a request querying by title
        let res = fetch_many_comments_service.exec(
            false,
            FetchManyCommentsParams {
                page: Some(2),
                per_page: Some(1),
                query: Some(ServiceCommentQueryType::Content("comment".to_string()))
            }
        ).await.unwrap();

        assert_eq!(1, res.data.len());
        assert_eq!(res.pagination, PaginationResponse { current_page: 2, total_pages: 2, total_items: 2 });
        assert_eq!(res.data[0].content(), "Comment 2 content here");

        // make a request with no query
        let res_2 = fetch_many_comments_service.exec(
            false,
            FetchManyCommentsParams {
                page: None,
                per_page: None,
                query: None,
            }
        ).await.unwrap();

        assert_eq!(2, res_2.data.len());
        assert_eq!(res_2.pagination, PaginationResponse { current_page: 1, total_pages: 1, total_items: 2 });
        assert_eq!(res_2.data[0].content(), "Comment 1 content here");
        assert_eq!(res_2.data[1].content(), "Comment 2 content here");

        // make a request querying by nickname that does not exist
        let res_3 = fetch_many_comments_service.exec(
            false,
            FetchManyCommentsParams {
                page: None,
                per_page: None,
                query: Some(ServiceCommentQueryType::Author("Vamp".to_string())),
            }
        ).await.unwrap_err();

        assert_eq!(res_3.code(), &StatusCode::NOT_FOUND);

        // make a request querying by nickname that exists and include inactive comments
        let res_4 = fetch_many_comments_service.exec(
            true,
            FetchManyCommentsParams {
                page: None,
                per_page: None,
                query: Some(ServiceCommentQueryType::Author("Floricultor".to_string())),
            }
        ).await.unwrap();

        assert_eq!(3, res_4.data.len());
        assert_eq!(res_4.pagination, PaginationResponse { current_page: 1, total_pages: 1, total_items: 3 });

        /* RESPONSE SAMPLE

        FetchManyCommentsResponse {
            pagination: PaginationResponse {
                current_page: 2,
                total_pages: 2,
                total_items: 2,
            },
            data: [
                Comment {
                    ...
                },
            ],
        }

        */
    }
}