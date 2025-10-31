use uuid::Uuid;

use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_comment_repository::{
    ArticleCommentRepositoryTrait,
    CommentQueryType,
    FindManyCommentsResponse,
};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceCommentQueryType {
    Author(String),
    Content(String),
}

pub struct FetchManyCommentsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<ServiceCommentQueryType>,
}

pub struct FetchManyCommentsService<ArticleCommentRepository, UserRepository>
where
    ArticleCommentRepository: ArticleCommentRepositoryTrait,
    UserRepository: UserRepositoryTrait,
{
    article_comment_repository: ArticleCommentRepository,
    user_repository: UserRepository,
}

#[derive(Debug)]
pub struct FetchManyCommentsResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<Comment>,
}

type ExecFuncReturn = Result<FetchManyCommentsResponse, SamambaiaError>;

impl<ArticleCommentRepository: ArticleCommentRepositoryTrait, UserRepository: UserRepositoryTrait>
    FetchManyCommentsService<ArticleCommentRepository, UserRepository>
{
    // CONSTRUCTOR
    pub fn new(
        article_comment_repository: ArticleCommentRepository,
        user_repository: UserRepository,
    ) -> Self {
        FetchManyCommentsService {
            article_comment_repository,
            user_repository,
        }
    }

    pub async fn exec_with_article_id(
        &self,
        article_id: Uuid,
        include_inactive: bool,
        params: FetchManyCommentsParams,
    ) -> ExecFuncReturn {
        self.fetch(Some(article_id), include_inactive, params).await
    }

    pub async fn exec(
        &self,
        include_inactive: bool,
        params: FetchManyCommentsParams,
    ) -> ExecFuncReturn {
        self.fetch(None, include_inactive, params).await
    }

    async fn fetch(
        &self,
        article_id: Option<Uuid>,
        include_inactive: bool,
        params: FetchManyCommentsParams,
    ) -> ExecFuncReturn {
        let default_items_per_page = 9;
        let default_page = 1;

        let items_per_page = if params.per_page.is_some() {
            params.per_page.unwrap()
        } else {
            default_items_per_page
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

        let query = self.parse_query(params.query).await?;

        let response = self
            .article_comment_repository
            .find_many_comments(
                article_id,
                include_inactive,
                PaginationParameters {
                    items_per_page,
                    page,
                    query,
                },
            )
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Fetch Many Comments Service, while fetching many comments from database",
                err,
            ))?;

        let FindManyCommentsResponse(comments, total_items) = response;

        Ok(FetchManyCommentsResponse {
            data: comments,
            pagination: PaginationResponse::new(page, total_items, items_per_page),
        })
    }

    async fn parse_query(
        &self,
        query: Option<ServiceCommentQueryType>,
    ) -> Result<Option<CommentQueryType>, SamambaiaError> {
        if query.is_none() {
            return Ok(None);
        }

        let query = query.unwrap();

        match query {
            ServiceCommentQueryType::Author(content) => {
                let user = self.user_repository.find_by_nickname(&content).await;

                if user.is_err() {
                    return Err(SamambaiaError::internal_err());
                }

                let user = user.unwrap();

                if user.is_none() {
                    return Err(SamambaiaError::resource_not_found_err());
                }

                let content = user.unwrap().id();

                Ok(Some(CommentQueryType::Author(content)))
            }
            ServiceCommentQueryType::Content(content) => {
                Ok(Some(CommentQueryType::Content(content)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use http::StatusCode;
    use tokio;

    use super::*;
    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::article_tag::DraftArticleTag;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
    use crate::libs::time::TimeHelper;
    use crate::tests::relationship_managers::comment_article::CommentArticleRelationInMemoryManager;
    use crate::tests::repositories::article_comment_repository::get_article_comment_repository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;
    use crate::tests::repositories::comment_repository::get_comment_repository;
    use crate::tests::repositories::users_repository::get_user_repository;

    #[tokio::test]
    async fn test() {
        let comment_article_relationship_manager =
            Arc::new(CommentArticleRelationInMemoryManager::new());

        let article_tag_repository = InMemoryArticleTagRepository::default();
        let (_, user_repository) = get_user_repository(None);
        let (_article_db, comment_db, article_comment_repository) = get_article_comment_repository(
            None,
            None,
            comment_article_relationship_manager.clone(),
        );

        let (_, comment_repository) = get_comment_repository(
            Some(comment_db),
            comment_article_relationship_manager.clone(),
        );

        let user = user_repository
            .create(User::new(
                "Floricultor".to_string(),
                "password".to_string(),
                Some(Role::Principal),
            ))
            .await
            .unwrap();

        let foo_tag = article_tag_repository
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();

        let article = Article::new(
            user.id(),
            "Título da notícia".into(),
            "Conteúdo da notícia".into(),
            "url do cover".into(),
            "baz".into(),
            vec![foo_tag.clone()],
            None,
        );

        comment_repository
            .create(Comment::new(
                user.id(),
                Some(article.id()),
                "Comment 1 content here".to_string(),
            ))
            .await
            .unwrap();

        comment_repository
            .create(Comment::new(
                user.id(),
                Some(article.id()),
                "Comment 2 content here".to_string(),
            ))
            .await
            .unwrap();

        comment_repository
            .create(Comment::new_from_existing(
                Uuid::new_v4(),
                Some(article.id()),
                user.id(),
                "Coment 2 content here".into(),
                false,
                TimeHelper::now(),
            ))
            .await
            .unwrap();

        let fetch_many_comments_service =
            FetchManyCommentsService::new(article_comment_repository, user_repository);

        // make a request querying by title
        let res = fetch_many_comments_service
            .exec(
                false,
                FetchManyCommentsParams {
                    page: Some(2),
                    per_page: Some(1),
                    query: Some(ServiceCommentQueryType::Content("comment".to_string())),
                },
            )
            .await
            .unwrap();

        assert_eq!(1, res.data.len());
        assert_eq!(
            res.pagination,
            PaginationResponse {
                current_page: 2,
                total_pages: 2,
                total_items: 2
            }
        );
        assert_eq!(res.data[0].content(), "Comment 2 content here");

        // make a request with no query
        let res_2 = fetch_many_comments_service
            .exec(
                false,
                FetchManyCommentsParams {
                    page: None,
                    per_page: None,
                    query: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(2, res_2.data.len());
        assert_eq!(
            res_2.pagination,
            PaginationResponse {
                current_page: 1,
                total_pages: 1,
                total_items: 2
            }
        );
        assert_eq!(res_2.data[0].content(), "Comment 1 content here");
        assert_eq!(res_2.data[1].content(), "Comment 2 content here");

        // make a request querying by nickname that does not exist
        let res_3 = fetch_many_comments_service
            .exec(
                false,
                FetchManyCommentsParams {
                    page: None,
                    per_page: None,
                    query: Some(ServiceCommentQueryType::Author("Vamp".to_string())),
                },
            )
            .await
            .unwrap_err();

        assert_eq!(res_3.get_code(), StatusCode::NOT_FOUND);

        // make a request querying by nickname that exists and include inactive comments
        let res_4 = fetch_many_comments_service
            .exec(
                true,
                FetchManyCommentsParams {
                    page: None,
                    per_page: None,
                    query: Some(ServiceCommentQueryType::Author("Floricultor".to_string())),
                },
            )
            .await
            .unwrap();

        assert_eq!(3, res_4.data.len());
        assert_eq!(
            res_4.pagination,
            PaginationResponse {
                current_page: 1,
                total_pages: 1,
                total_items: 3
            }
        );
    }
}
