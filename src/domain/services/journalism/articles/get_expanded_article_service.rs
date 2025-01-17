use crate::core::pagination::PaginationParameters;
use crate::core::pagination::PaginationResponse;
use crate::core::pagination::DEFAULT_PER_PAGE;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::slug::Slug;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::comment_user_article_repository::CommentUserArticleRepositoryTrait;
use crate::domain::repositories::comment_user_article_repository::FindManyCommentsWithAuthorResponse;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};
use uuid::Uuid;

pub struct GetExpandedArticleParams<'exec> {
    pub article_slug: Slug,
    pub comments_per_page: Option<u32>,
    pub user_role: Option<&'exec Role>,
    pub user_id: Option<&'exec Uuid>,
}

#[derive(Debug)]
pub struct FetchManyCommentsWithAuthorResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<CommentWithAuthor>,
}

#[derive(Debug)]
pub struct GetExpandedArticleResponse {
    pub article: Article,
    pub article_author: User,
    pub comments: FetchManyCommentsWithAuthorResponse,
}

pub struct GetExpandedArticleService<UR, AR, CUAR>
where
    UR: UserRepositoryTrait,
    AR: ArticleRepositoryTrait,
    CUAR: CommentUserArticleRepositoryTrait,
{
    user_repository: UR,
    article_repository: AR,
    comment_user_article_repository: CUAR,
}

impl<
        UR: UserRepositoryTrait,
        AR: ArticleRepositoryTrait,
        CUAR: CommentUserArticleRepositoryTrait,
    > GetExpandedArticleService<UR, AR, CUAR>
{
    pub fn new(
        user_repository: UR,
        article_repository: AR,
        comment_user_article_repository: CUAR,
    ) -> Self {
        GetExpandedArticleService {
            user_repository,
            article_repository,
            comment_user_article_repository,
        }
    }

    pub async fn exec(
        &self,
        params: GetExpandedArticleParams<'_>,
    ) -> Result<GetExpandedArticleResponse, SamambaiaError> {
        let items_per_page = params.comments_per_page.unwrap_or(DEFAULT_PER_PAGE as u32);

        let article = match self
            .article_repository
            .find_by_slug(&params.article_slug)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Get Expanded Article Service, while finding article by Id",
                    err,
                )
            })? {
            None => return Err(SamambaiaError::resource_not_found_err()),
            Some(article) => article,
        };

        let user_can_see_article = {
            if params.user_id.is_none() || params.user_role.is_none() {
                false
            } else if article.author_id().eq(params.user_id.unwrap()) {
                true
            } else {
                verify_role_has_permission(
                    params.user_role.unwrap(),
                    RolePermissions::SeeUnapprovedArticle,
                )
            }
        };

        if !article.approved() && !user_can_see_article {
            return Err(SamambaiaError::resource_not_found_err());
        }

        let FindManyCommentsWithAuthorResponse(data, total_items) = self
            .comment_user_article_repository
            .find_many_comments(
                article.id(),
                false,
                PaginationParameters {
                    items_per_page,
                    page: 1,
                    query: None,
                },
            )
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Get Expanded Article Service, while fetching many comments by article id",
                   err,
                )
            })?;

        let comments = FetchManyCommentsWithAuthorResponse {
            data,
            pagination: PaginationResponse::new(1, total_items, items_per_page),
        };

        let author = self
            .user_repository
            .find_by_id(&article.author_id())
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Get Expanded Article Service, while finding User by id",
                    err,
                )
            })?;

        if author.is_none() {
            log::error!(
                "Author from article of id '{}' returned None on Get Expanded Article Service.",
                article.id().to_string()
            );
            return Err(SamambaiaError::resource_not_found_err());
        }

        let author = author.unwrap();

        Ok(GetExpandedArticleResponse {
            article,
            article_author: author,
            comments,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::{Arc, Mutex};
    use tokio;
    use uuid::Uuid;

    use crate::domain::domain_entities::{comment_with_author::CommentWithAuthor, role::Role};
    use crate::domain::repositories::comment_user_article_repository::{
        CommentWithAuthorQueryType, MockCommentUserArticleRepositoryTrait,
    };
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::article_repository::get_article_repository;

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo = MockUserRepositoryTrait::new();
        let mut mock_comm_user_art_repo = MockCommentUserArticleRepositoryTrait::new();
        let (articles_db, _, mocked_article_repository) = get_article_repository();

        let comments_db: Arc<Mutex<Vec<CommentWithAuthor>>> = Arc::new(Mutex::new(vec![]));

        // POPULATING
        let mocked_article = Article::new(
            Uuid::new_v4(),
            "Notícia 1".into(),
            "Conteúdo da notícia 1.".into(),
            "url_da_cover.com".into(),
            1,
            "MockedTag".into(),
            "Mocked description".into(),
        );

        let mocked_article_id = mocked_article.id();
        let mocked_article_slug = mocked_article.slug().clone();
        articles_db.lock().unwrap().push(mocked_article);

        let mocked_comm_1 = CommentWithAuthor::new(
            Some(mocked_article_id),
            "comentario 1 conteudo".into(),
            User::new("Salem".into(), "123".into(), Some(Role::User)),
        );

        let mocked_comm_2 = CommentWithAuthor::new(
            Some(mocked_article_id),
            "comentario 2 conteudo".into(),
            User::new("Elffi".into(), "123".into(), Some(Role::User)),
        );

        comments_db.lock().unwrap().push(mocked_comm_1.clone());
        comments_db.lock().unwrap().push(mocked_comm_2.clone());

        let user = User::new_from_existing(
            Uuid::new_v4(),
            "Floricultor".into(),
            "123".into(),
            TimeHelper::now(),
            None,
            Some(Role::Ceo),
        );

        let user_id = user.id();

        // MOCKING REPOSITORIES
        mocked_user_repo
            .expect_find_by_id()
            .returning(move |_id| Ok(Some(user.clone())));

        let comments_db_to_move = Arc::clone(&comments_db);
        mock_comm_user_art_repo
            .expect_find_many_comments()
            .returning(move |_article_id, include_inactive, params| {
                let PaginationParameters {
                    page,
                    items_per_page,
                    query,
                } = params;

                let mut comments: Vec<CommentWithAuthor> = Vec::new();

                if query.is_some() {
                    match query.unwrap() {
                        CommentWithAuthorQueryType::Content(content) => {
                            for item in comments_db_to_move.lock().unwrap().iter() {
                                if item
                                    .content()
                                    .to_lowercase()
                                    .contains(&content.to_lowercase()[..])
                                    || include_inactive
                                    || item.is_active()
                                {
                                    comments.push(item.clone());
                                }
                            }
                        }
                        CommentWithAuthorQueryType::Author(content) => {
                            for item in comments_db_to_move.lock().unwrap().iter() {
                                if item.author().id().eq(&content)
                                    || include_inactive
                                    || item.is_active()
                                {
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

                Ok(FindManyCommentsWithAuthorResponse(
                    res_comments,
                    total_of_items_before_paginating as u64,
                ))
            });

        let sut = GetExpandedArticleService {
            user_repository: mocked_user_repo,
            comment_user_article_repository: mock_comm_user_art_repo,
            article_repository: mocked_article_repository,
        };

        let allowed_result = sut
            .exec(GetExpandedArticleParams {
                article_slug: mocked_article_slug.clone(),
                comments_per_page: None,
                user_id: Some(&user_id),
                user_role: Some(&Role::Editor),
            })
            .await
            .unwrap();

        let GetExpandedArticleResponse {
            article,
            article_author,
            comments,
        } = allowed_result;

        let FetchManyCommentsWithAuthorResponse { data, pagination } = comments;

        assert_eq!(mocked_comm_1, data[0].clone());
        assert_eq!(mocked_comm_2, data[1].clone());
        assert_eq!(2, pagination.total_items);
        assert_eq!(mocked_article_id, article.id());
        assert_eq!(user_id, article_author.id());

        let unauthorized_result = sut
            .exec(GetExpandedArticleParams {
                article_slug: mocked_article_slug.clone(),
                comments_per_page: None,
                user_id: None,
                user_role: None,
            })
            .await;

        assert!(
            unauthorized_result.is_err(),
            "Expected a user not to be able to see an unapproved article if it's not the author and nor has the permission to see unapproved articles."
        );
    }
}
