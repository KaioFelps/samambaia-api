use uuid::Uuid;

use crate::core::pagination::{DEFAULT_PER_PAGE, PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::comment_user_article_repository::{
    CommentUserArticleRepositoryTrait,
    FindManyCommentsWithAuthorResponse,
};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::domain::value_objects::slug::Slug;
use crate::error::SamambaiaError;
use crate::util::{RolePermissions, generate_service_internal_error, verify_role_has_permission};

pub struct GetExpandedArticleParams<'exec> {
    pub article_slug: Slug,
    pub comments_per_page: Option<u32>,
    pub comments_page: Option<u32>,
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

impl<UR, AR, CUAR> GetExpandedArticleService<UR, AR, CUAR>
where
    UR: UserRepositoryTrait,
    AR: ArticleRepositoryTrait,
    CUAR: CommentUserArticleRepositoryTrait,
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

        let comments_per_page = params.comments_per_page.unwrap_or(DEFAULT_PER_PAGE as u32);
        let comments_page = params.comments_page.unwrap_or(1);

        let FindManyCommentsWithAuthorResponse(data, total_items) = self
            .comment_user_article_repository
            .find_many_comments(
                article.id(),
                false,
                PaginationParameters {
                    items_per_page: comments_per_page,
                    page: comments_page,
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
            pagination: PaginationResponse::new(comments_page, total_items, comments_per_page),
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
                article.id()
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
    use tokio;
    use uuid::Uuid;

    use super::*;
    use crate::domain::domain_entities::article_tag::DraftArticleTag;
    use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;
    use crate::tests::repositories::comment_with_user_repository::InMemoryCommentUserRepository;
    use crate::tests::repositories::users_repository::get_user_repository;

    #[tokio::test]
    async fn test() {
        let mock_comm_user_art_repo = InMemoryCommentUserRepository::default();
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let mocked_article_repository =
            InMemoryArticleRepository::default(article_tag_repository.clone());
        let (_, mocked_user_repo) =
            get_user_repository(Some(mocked_article_repository.user_db.clone()));

        let tag = article_tag_repository
            .create(DraftArticleTag::new("MockedTag".into()))
            .await
            .unwrap();

        // region: --- populating
        let user = User::new_from_existing(
            Uuid::new_v4(),
            "Floricultor".into(),
            "123".into(),
            TimeHelper::now(),
            None,
            Some(Role::Ceo),
        );

        mocked_article_repository
            .user_db
            .lock()
            .unwrap()
            .push(user.clone());

        let mocked_article = Article::new(
            user.id(),
            "Notícia 1".into(),
            "Conteúdo da notícia 1.".into(),
            "url_da_cover.com".into(),
            "Mocked description".into(),
            vec![tag.clone()],
            None,
            None,
        );

        let mocked_article_id = mocked_article.id();
        let mocked_article_slug = mocked_article.slug().clone();
        mocked_article_repository
            .article_db
            .lock()
            .unwrap()
            .push(mocked_article);

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

        mock_comm_user_art_repo
            .comment_with_author_db
            .lock()
            .unwrap()
            .push(mocked_comm_1.clone());

        mock_comm_user_art_repo
            .comment_with_author_db
            .lock()
            .unwrap()
            .push(mocked_comm_2.clone());
        // endregion: --- populating

        let sut = GetExpandedArticleService {
            user_repository: mocked_user_repo,
            comment_user_article_repository: mock_comm_user_art_repo,
            article_repository: mocked_article_repository.clone(),
        };

        let allowed_result = sut
            .exec(GetExpandedArticleParams {
                article_slug: mocked_article_slug.clone(),
                comments_per_page: None,
                comments_page: None,
                user_id: Some(&user.id()),
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
        assert_eq!(user.id(), article_author.id());

        let unauthorized_result = sut
            .exec(GetExpandedArticleParams {
                article_slug: mocked_article_slug.clone(),
                comments_per_page: None,
                user_id: None,
                comments_page: None,
                user_role: None,
            })
            .await;

        assert!(
            unauthorized_result.is_err(),
            "Expected a user not to be able to see an unapproved article if it's not the author and nor has the permission to see unapproved articles."
        );
    }
}
