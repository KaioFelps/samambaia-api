use std::error::Error;
use log::{error, info};
use uuid::Uuid;

use crate::core::pagination::PaginationResponse;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::comment_user_article_repository::FindManyCommentsWithAuthorResponse;
use crate::domain::repositories::comment_user_article_repository::CommentUserArticleRepositoryTrait;
use crate::core::pagination::PaginationParameters;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;

use crate::{R_EOL, LOG_SEP};

pub struct GetExpandedArticleParams {
    pub article_id: Uuid
}

#[derive(Debug)]
pub struct FetchManyCommentsWithAuthorResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<CommentWithAuthor>
}

#[derive(Debug)]
pub struct GetExpandedArticleResponse {
    pub article: Article,
    pub article_author: User,
    pub comments: FetchManyCommentsWithAuthorResponse,
}

pub struct GetExpandedArticleService<UR, AR, CUAR>
where   UR: UserRepositoryTrait,
        AR: ArticleRepositoryTrait,
        CUAR: CommentUserArticleRepositoryTrait
        {
    user_repository: Box<UR>,
    article_repository: Box<AR>,
    comment_user_article_repository: Box<CUAR>
}

impl<
    UR: UserRepositoryTrait,
    AR: ArticleRepositoryTrait,
    CUAR: CommentUserArticleRepositoryTrait
> GetExpandedArticleService<UR, AR, CUAR> {
    pub fn new(
        user_repository: Box<UR>,
        article_repository: Box<AR>,
        comment_user_article_repository: Box<CUAR>
    ) -> Self {
        GetExpandedArticleService {
            user_repository,
            article_repository,
            comment_user_article_repository
        }
    }

    pub async fn exec(&self, params: GetExpandedArticleParams) -> Result<GetExpandedArticleResponse, Box<dyn Error>> {
        let items_per_page = 5;

        let article = self.article_repository.find_by_id(params.article_id).await;

        if article.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Get Expanded Article Service, while finding article by Id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                article.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let article = article.unwrap();

        if article.is_none() {
            info!("{R_EOL}{LOG_SEP}{R_EOL}Article returned None on Get Expanded Article Service.{R_EOL}{LOG_SEP}{R_EOL}");

            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let article = article.unwrap();

        let comments = self.comment_user_article_repository.find_many_comments(
            article.id(),
            PaginationParameters {
                items_per_page,
                page: 1,
                query: None,
            }
        ).await;

        if comments.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Get Expanded Article Service, while fetching many comments by article id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                comments.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let FindManyCommentsWithAuthorResponse (data, total_items) = comments.unwrap();

        let comments = FetchManyCommentsWithAuthorResponse {
            data,
            pagination: PaginationResponse {
                current_page: 1,
                total_items,
                total_pages: (total_items as f64 / items_per_page as f64).ceil() as u32
            }
        };

        let author = self.user_repository.find_by_id(&article.author_id()).await;

        if author.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Get Expanded Article Service, while finding User by id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                author.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let author = author.unwrap();

        if author.is_none() {
            error!("Author returned None on Get Expanded Article Service.");

            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let author = author.unwrap();

        Ok(GetExpandedArticleResponse {
            article,
            article_author: author,
            comments
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use tokio;
    use chrono::Utc;
    use std::sync::{Arc, Mutex};
    
    use crate::domain::domain_entities::{comment_with_author::CommentWithAuthor, role::Role}; 
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::repositories::comment_user_article_repository::{CommentWithAuthorQueryType, MockCommentUserArticleRepositoryTrait};
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo = MockUserRepositoryTrait::new();
        let mut mock_comm_user_art_repo = MockCommentUserArticleRepositoryTrait::new();
        let mut mocked_article_repository = MockArticleRepositoryTrait::new();

        let articles_db: Arc<Mutex<Vec<Article>>> = Arc::new(Mutex::new(vec![]));
        let comments_db: Arc<Mutex<Vec<CommentWithAuthor>>> = Arc::new(Mutex::new(vec![]));

        
        // POPULATING
        let mocked_article = Article::new(
            Uuid::new_v4(),
            "Notícia 1".into(),
            "Conteúdo da notícia 1.".into(),
            "url_da_cover.com".into(),
        );

        let mocked_article_id = mocked_article.id();
        articles_db.lock().unwrap().push(mocked_article);

        let mocked_comm_1 = CommentWithAuthor::new(
            mocked_article_id.clone(),
            "comentario 1 conteudo".into(),
            User::new("Salem".into(), "123".into(), Some(Role::User))
        );

        let mocked_comm_2 = CommentWithAuthor::new(
            mocked_article_id.clone(),
            "comentario 2 conteudo".into(),
            User::new("Elffi".into(), "123".into(), Some(Role::User))
        );

        comments_db.lock().unwrap().push(mocked_comm_1.clone());
        comments_db.lock().unwrap().push(mocked_comm_2.clone());

        let user = User::new_from_existing(
            Uuid::new_v4(),
            "Floricultor".into(),
            "123".into(),
            Utc::now().naive_utc(),
            None,
            Some(Role::Ceo)
        );

        let user_id = user.id();

        // MOCKING REPOSITORIES
        mocked_user_repo
        .expect_find_by_id()
        .returning(move |_id| {
            Ok(Some(user.clone()))
        });


        let articles_db_to_move = Arc::clone(&articles_db);
        mocked_article_repository
        .expect_find_by_id()
        .returning(move |article_id| {
            let mut article: Option<Article> = None;

            for item in articles_db_to_move.lock().unwrap().iter() {
                if item.id().eq(&article_id) {
                    article = Some(item.clone());
                    break;
                }
            }

            Ok(article)
        });
        
        let comments_db_to_move = Arc::clone(&comments_db);
        mock_comm_user_art_repo
        .expect_find_many_comments()
        .returning(move |_article_id, params| {
            let PaginationParameters { page, items_per_page, query } = params;

            let mut comments: Vec<CommentWithAuthor> = Vec::new();

            if query.is_some() {
                match query.unwrap() {
                    CommentWithAuthorQueryType::CONTENT(content) => {
                        for item in comments_db_to_move.lock().unwrap().iter() {
                            if item.content().to_lowercase().contains(&content.to_lowercase()[..]) {
                                comments.push(item.clone());
                            }
                        }
                    },
                    CommentWithAuthorQueryType::AUTHOR(content) => {
                        for item in comments_db_to_move.lock().unwrap().iter() {
                            if item.author().id().eq(&content) {
                                comments.push(item.clone());
                            }
                        }
                    }
                }
            } else {
                comments = comments_db_to_move.lock().unwrap().clone();
            }

            let total_of_items_before_paginating = comments.len();

            let leap = (page - 1) * items_per_page;

            let mut res_comments = vec![];

            for (index, item) in comments.iter().enumerate() {
                if index >= leap as usize {
                    res_comments.push(item.to_owned());
                }
            }

            Ok(FindManyCommentsWithAuthorResponse (res_comments, total_of_items_before_paginating as u64))
        });
        
        let sut = GetExpandedArticleService {
            user_repository: Box::new(mocked_user_repo),
            comment_user_article_repository: Box::new(mock_comm_user_art_repo),
            article_repository: Box::new(mocked_article_repository),
        };

        let res = sut.exec(GetExpandedArticleParams {
            article_id: mocked_article_id
        }).await.unwrap();

        let GetExpandedArticleResponse {
        article,
        article_author,
        comments
        } = res;

        let FetchManyCommentsWithAuthorResponse {
            data,
            pagination
        } = comments;

        assert_eq!(mocked_comm_1, data[0].clone());
        assert_eq!(mocked_comm_2, data[1].clone());
        assert_eq!(2, pagination.total_items);
        assert_eq!(mocked_article_id, article.id());
        assert_eq!(user_id, article_author.id());
    }
}
