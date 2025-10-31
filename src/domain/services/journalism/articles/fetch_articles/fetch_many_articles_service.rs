use super::params::FetchArticlesParams;
use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::{
    ArticleRepositoryTrait,
    FindManyArticlesResponse,
};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

type Error = SamambaiaError;

pub struct FetchManyArticlesService<ArticleRepository, UserRepository>
where
    ArticleRepository: ArticleRepositoryTrait,
    UserRepository: UserRepositoryTrait,
{
    article_repository: ArticleRepository,
    user_repository: UserRepository,
}

#[derive(Debug)]
pub struct FetchManyArticlesResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<Article>,
}

impl<ArticleRepository: ArticleRepositoryTrait, UserRepository: UserRepositoryTrait>
    FetchManyArticlesService<ArticleRepository, UserRepository>
{
    // CONSTRUCTOR
    pub fn new(article_repository: ArticleRepository, user_repository: UserRepository) -> Self {
        FetchManyArticlesService {
            article_repository,
            user_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchArticlesParams,
    ) -> Result<FetchManyArticlesResponse, Error> {
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

        let query = super::parse_article_fetch_query(&self.user_repository, params.query).await?;

        let FindManyArticlesResponse(articles, total_items) = self
            .article_repository
            .find_many(
                PaginationParameters {
                    items_per_page,
                    page,
                    query,
                },
                params.approved_state,
            )
            .await
            .map_err(|err|
                generate_service_internal_error(
                    "Error occurred on Fetch Many Articles Service, while finding many articles from database",
                    err,
                )
            )?;

        Ok(FetchManyArticlesResponse {
            data: articles,
            pagination: PaginationResponse::new(page, total_items, items_per_page),
        })
    }
}

#[cfg(test)]
mod test {
    use http::StatusCode;
    use tokio;

    use super::*;
    use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::domain::services::journalism::articles::fetch_articles::params::FetchArticleQuery;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;
    use crate::tests::repositories::users_repository::get_user_repository;

    async fn create_article_with_tag<AR, UR, ATR>(
        article_repo: &AR,
        user_repo: &UR,
        tag_repo: &ATR,
    ) -> (Article, ArticleTag, User)
    where
        AR: ArticleRepositoryTrait,
        UR: UserRepositoryTrait,
        ATR: ArticleTagRepositoryTrait,
    {
        let foo_tag = tag_repo
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();

        let user = User::new(
            "Floricultor".to_string(),
            "password".to_string(),
            Some(Role::Principal),
        );

        let user = user_repo.create(user).await.unwrap();

        let mut approved_article = Article::new(
            user.id(),
            "Article 1 title".to_string(),
            "Article 1 content here".to_string(),
            "url".to_string(),
            "Description".into(),
            vec![foo_tag.clone()],
        );
        approved_article.set_approved(true);

        let approved_article = article_repo.create(approved_article).await.unwrap();
        article_repo
            .create(Article::new(
                user.id(),
                "Article 2 title".to_string(),
                "Article 2 content here".to_string(),
                "url".to_string(),
                "Description".into(),
                vec![foo_tag.clone()],
            ))
            .await
            .unwrap();

        (approved_article, foo_tag, user)
    }

    #[tokio::test]
    async fn should_fetch_many_articles() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());
        let (_, mocked_user_repo) = get_user_repository(None);

        let (_, foo_tag, _) = create_article_with_tag(
            &article_repository,
            &mocked_user_repo,
            &article_tag_repository,
        )
        .await;

        let fetch_many_articles_service =
            FetchManyArticlesService::new(article_repository, mocked_user_repo);

        let query_by_title_request = fetch_many_articles_service
            .exec(FetchArticlesParams {
                page: Some(2),
                per_page: Some(1),
                query: Some(FetchArticleQuery::Title("article".to_string())),
                approved_state: None,
            })
            .await
            .unwrap();

        assert_eq!(
            1,
            query_by_title_request.data.len(),
            "Expected exactly one article with the queried title."
        );

        assert_eq!(
            query_by_title_request.pagination,
            PaginationResponse {
                current_page: 2,
                total_pages: 2,
                total_items: 2
            }
        );

        assert_eq!(
            query_by_title_request.data[0].title(),
            "Article 2 title",
            "Expected queried article to have title \"Article 2 title\"."
        );

        assert!(
            query_by_title_request.data[0]
                .get_tags()
                .contains(&&foo_tag)
        );
    }

    #[tokio::test]
    async fn should_fetch_many_articles_without_filters() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());
        let (_, mocked_user_repo) = get_user_repository(None);

        create_article_with_tag(
            &article_repository,
            &mocked_user_repo,
            &article_tag_repository,
        )
        .await;

        let sut = FetchManyArticlesService::new(article_repository, mocked_user_repo);

        let no_query_request = sut
            .exec(FetchArticlesParams {
                page: None,
                per_page: None,
                query: None,
                approved_state: None,
            })
            .await
            .unwrap();

        assert_eq!(
            2,
            no_query_request.data.len(),
            "Expected to get all the 2 existing articles."
        );
        assert_eq!(
            no_query_request.pagination,
            PaginationResponse {
                current_page: 1,
                total_pages: 1,
                total_items: 2
            }
        );
        assert_eq!(no_query_request.data[0].title(), "Article 1 title");
        assert_eq!(no_query_request.data[1].title(), "Article 2 title");
    }

    #[tokio::test]
    async fn should_handle_non_existing_nickname_queries() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());
        let (_, mocked_user_repo) = get_user_repository(None);

        create_article_with_tag(
            &article_repository,
            &mocked_user_repo,
            &article_tag_repository,
        )
        .await;

        let sut = FetchManyArticlesService::new(article_repository, mocked_user_repo);

        let failing_query_by_unexisting_nickname_request = sut
            .exec(FetchArticlesParams {
                page: None,
                per_page: None,
                query: Some(FetchArticleQuery::Author("Vamp".to_string())),
                approved_state: None,
            })
            .await
            .unwrap_err();

        assert_eq!(
            failing_query_by_unexisting_nickname_request.get_code(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn should_find_articles_filtered_by_nickname() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());
        let (_, mocked_user_repo) = get_user_repository(None);

        create_article_with_tag(
            &article_repository,
            &mocked_user_repo,
            &article_tag_repository,
        )
        .await;

        let sut = FetchManyArticlesService::new(article_repository, mocked_user_repo);

        // make a request querying by nickname that exists
        let query_by_nickname_request = sut
            .exec(FetchArticlesParams {
                page: None,
                per_page: None,
                query: Some(FetchArticleQuery::Author("Floricultor".to_string())),
                approved_state: None,
            })
            .await
            .unwrap();

        assert_eq!(2, query_by_nickname_request.data.len());
        assert_eq!(
            query_by_nickname_request.pagination,
            PaginationResponse {
                current_page: 1,
                total_pages: 1,
                total_items: 2
            }
        );
    }

    #[tokio::test]
    async fn should_be_able_to_show_only_approved_articles() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());
        let (_, mocked_user_repo) = get_user_repository(None);

        create_article_with_tag(
            &article_repository,
            &mocked_user_repo,
            &article_tag_repository,
        )
        .await;

        let sut = FetchManyArticlesService::new(article_repository, mocked_user_repo);

        let result = sut
            .exec(FetchArticlesParams {
                page: None,
                per_page: None,
                query: None,
                approved_state: Some(true),
            })
            .await
            .unwrap();

        assert_eq!(
            1,
            result.data.len(),
            "Expected only-approved-articles request to be 1 item length."
        );
        assert_eq!(
            1, result.pagination.total_items,
            "Expected only-approved-articles request pagination total_items to be 1."
        )
    }
}
