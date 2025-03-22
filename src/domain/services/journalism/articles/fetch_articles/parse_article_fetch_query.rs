use super::params::FetchArticleQuery;
use crate::domain::repositories::article_repository::ArticleQueryType;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub async fn parse_article_fetch_query<UR: UserRepositoryTrait>(
    user_repository: &UR,
    query: Option<FetchArticleQuery>,
) -> Result<Option<ArticleQueryType>, SamambaiaError> {
    let query = match query {
        None => return Ok(None),
        Some(query) => query,
    };

    match query {
        FetchArticleQuery::Title(title) => Ok(Some(ArticleQueryType::Title(title))),
        FetchArticleQuery::Author(author) => {
            match user_repository
                .find_by_nickname(&author)
                .await
                .map_err(|err| {
                    generate_service_internal_error("Failed to fetch user from repository while parsing article fetch service query", err)
                })?
                .map(|user| Some(ArticleQueryType::Author(user.id()))) {
                    None => Err(SamambaiaError::resource_not_found_err()),
                    Some(author_query) => Ok(author_query)
                }
            }
        }
}
