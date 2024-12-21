use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::{
    ArticleQueryType, ArticleRepositoryTrait, FindManyArticlesResponse,
};
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::DomainError;
use crate::util::generate_service_internal_error;

type Error = DomainError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceArticleQueryType {
    Title(String),
    Author(String),
}

pub struct FetchManyArticlesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<ServiceArticleQueryType>,
    pub approved_state: Option<bool>,
}

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
        params: FetchManyArticlesParams,
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

        let query = self.parse_query(params.query).await?;

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

    async fn parse_query(
        &self,
        query: Option<ServiceArticleQueryType>,
    ) -> Result<Option<ArticleQueryType>, Error> {
        if query.is_none() {
            return Ok(None);
        }

        match query.unwrap() {
            ServiceArticleQueryType::Author(content) => {
                let user = self.user_repository.find_by_nickname(&content).await;

                if user.is_err() {
                    return Err(DomainError::internal_err());
                }

                let user = user.unwrap();

                if user.is_none() {
                    return Err(DomainError::resource_not_found_err());
                }

                Ok(Some(ArticleQueryType::Author(user.unwrap().id())))
            }
            ServiceArticleQueryType::Title(content) => Ok(Some(ArticleQueryType::Title(content))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use http::StatusCode;
    use tokio;

    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::tests::repositories::article_repository::get_article_repository;

    #[tokio::test]
    async fn test() {
        let (article_db, mocked_article_repo) = get_article_repository();
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();

        let user = User::new(
            "Floricultor".to_string(),
            "password".to_string(),
            Some(Role::Principal),
        );

        let mut approved_article = Article::new(
            user.id(),
            "Article 1 title".to_string(),
            "Article 1 content here".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
        );
        approved_article.set_approved(true);
        article_db.lock().unwrap().push(approved_article.clone());
        article_db.lock().unwrap().push(Article::new(
            user.id(),
            "Article 2 title".to_string(),
            "Article 2 content here".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
        ));

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

        let fetch_many_articles_service =
            FetchManyArticlesService::new(mocked_article_repo, mocked_user_repo);

        let query_by_title_request = fetch_many_articles_service
            .exec(FetchManyArticlesParams {
                page: Some(2),
                per_page: Some(1),
                query: Some(ServiceArticleQueryType::Title("article".to_string())),
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

        let no_query_request = fetch_many_articles_service
            .exec(FetchManyArticlesParams {
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

        // make a request querying by nickname that does not exist
        let failing_query_by_unexisting_nickname_request = fetch_many_articles_service
            .exec(FetchManyArticlesParams {
                page: None,
                per_page: None,
                query: Some(ServiceArticleQueryType::Author("Vamp".to_string())),
                approved_state: None,
            })
            .await
            .unwrap_err();

        assert_eq!(
            failing_query_by_unexisting_nickname_request.get_code(),
            StatusCode::NOT_FOUND
        );

        // make a request querying by nickname that exists
        let query_by_nickname_request = fetch_many_articles_service
            .exec(FetchManyArticlesParams {
                page: None,
                per_page: None,
                query: Some(ServiceArticleQueryType::Author("Floricultor".to_string())),
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

        let query_approved_only_articles_request = fetch_many_articles_service
            .exec(FetchManyArticlesParams {
                page: None,
                per_page: None,
                query: None,
                approved_state: Some(true),
            })
            .await
            .unwrap();

        assert_eq!(
            1,
            query_approved_only_articles_request.data.len(),
            "Expected only-approved-articles request to be 1 item length."
        );
        assert_eq!(
            1, query_approved_only_articles_request.pagination.total_items,
            "Expected only-approved-articles request pagination total_items to be 1."
        )
    }
}
