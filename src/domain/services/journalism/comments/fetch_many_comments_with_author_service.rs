use log::error;
use uuid::Uuid;

use crate::core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE};
use crate::domain::repositories::comment_user_article_repository::{
    CommentUserArticleRepositoryTrait, FindManyCommentsWithAuthorResponse,
};
use crate::error::DomainError;

use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
use crate::{LOG_SEP, R_EOL};

pub struct FetchManyArticleCommentsWithAuthorParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub struct FetchManyArticleCommentsWithAuthorService<CommentUserArticleRepository>
where
    CommentUserArticleRepository: CommentUserArticleRepositoryTrait,
{
    article_comment_repository: CommentUserArticleRepository,
}

#[derive(Debug)]
pub struct FetchManyCommentsWithAuthorResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<CommentWithAuthor>,
}

type ExecFuncReturn = Result<FetchManyCommentsWithAuthorResponse, DomainError>;

impl<CommentUserArticleRepository: CommentUserArticleRepositoryTrait>
    FetchManyArticleCommentsWithAuthorService<CommentUserArticleRepository>
{
    // CONSTRUCTOR
    pub fn new(article_comment_repository: CommentUserArticleRepository) -> Self {
        Self {
            article_comment_repository,
        }
    }

    pub async fn exec(
        &self,
        article_id: Uuid,
        params: FetchManyArticleCommentsWithAuthorParams,
    ) -> ExecFuncReturn {
        let default_page = 1;

        let items_per_page = if params.per_page.is_some() {
            params.per_page.unwrap()
        } else {
            DEFAULT_PER_PAGE as u32
        };

        let page = if params.page.is_some() {
            let params_page = params.page.unwrap();
            if params_page == 0 {
                default_page
            } else {
                params_page
            }
        } else {
            default_page
        };

        let response = self
            .article_comment_repository
            .find_many_comments(
                article_id,
                false,
                PaginationParameters {
                    items_per_page,
                    page,
                    query: None,
                },
            )
            .await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Comments With Author Service, while fetching many comments from database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(DomainError::internal_err());
        }

        let response = response.unwrap();
        let FindManyCommentsWithAuthorResponse(comments, total_items) = response;

        Ok(FetchManyCommentsWithAuthorResponse {
            data: comments,
            pagination: PaginationResponse::new(page, total_items, items_per_page),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio;

    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::comment_user_article_repository::MockCommentUserArticleRepositoryTrait;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::article_repository::get_article_repository;

    #[tokio::test]
    async fn test() {
        let mut mocked_comment_repo: MockCommentUserArticleRepositoryTrait =
            MockCommentUserArticleRepositoryTrait::new();
        let (article_db, _, _) = get_article_repository();

        let mut db: Vec<CommentWithAuthor> = Vec::new();

        let user = User::new(
            "Floricultor".to_string(),
            "password".to_string(),
            Some(Role::Principal),
        );
        let article = Article::new(
            user.id(),
            "Título da notícia".into(),
            "Conteúdo da notícia".into(),
            "url do cover".into(),
            1,
            "Foo".into(),
            "baz".into(),
        );
        let article_id = article.id();

        article_db.lock().unwrap().push(article.clone());

        db.push(CommentWithAuthor::new(
            Some(article.id()),
            "Comment 1 content here".to_string(),
            user.clone(),
        ));
        db.push(CommentWithAuthor::new(
            Some(article.id()),
            "Comment 2 content here".to_string(),
            user.clone(),
        ));
        db.push(CommentWithAuthor::new_from_existing(
            Uuid::new_v4(),
            Some(article.id()),
            "Comment 2 content here".into(),
            false,
            TimeHelper::now(),
            user.clone(),
        ));

        mocked_comment_repo.expect_find_many_comments().returning(
            move |_article_id, include_inactive, params| {
                let PaginationParameters {
                    page,
                    items_per_page,
                    query: _,
                } = params;

                let mut comments: Vec<CommentWithAuthor> = Vec::new();

                for item in db.iter() {
                    if include_inactive || item.is_active() {
                        comments.push(item.clone());
                    }
                }

                let total_of_items_before_paginating = comments.len();

                let leap = (page - 1) * items_per_page;

                let mut res_comments = vec![];

                for (index, item) in comments.iter().enumerate() {
                    if index >= leap as usize {
                        res_comments.push(item.to_owned());
                    }
                }

                Ok(FindManyCommentsWithAuthorResponse(
                    res_comments,
                    total_of_items_before_paginating as u64,
                ))
            },
        );

        let fetch_many_comments_service =
            FetchManyArticleCommentsWithAuthorService::new(mocked_comment_repo);

        let response = fetch_many_comments_service
            .exec(
                article_id,
                FetchManyArticleCommentsWithAuthorParams {
                    page: None,
                    per_page: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(2, response.data.len());
        assert_eq!(
            response.pagination,
            PaginationResponse {
                current_page: 1,
                total_pages: 1,
                total_items: 2
            }
        );
        assert_eq!(response.data[0].content(), "Comment 1 content here");
        assert_eq!(response.data[1].content(), "Comment 2 content here");
    }
}
