use super::params::FetchArticleQuery;
use crate::domain::repositories::article_repository::ArticleQueryType;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub async fn parse_article_fetch_query<UR: UserRepositoryTrait>(
    user_repository: &UR,
    queries: Vec<FetchArticleQuery>,
) -> Result<Vec<ArticleQueryType>, SamambaiaError> {
    let mut parsed_queries = vec![];
    for query in queries {
        match query {
            FetchArticleQuery::Title(title) => parsed_queries.push(ArticleQueryType::Title(title)),
            FetchArticleQuery::Author(author) => {
                match user_repository
                    .find_by_nickname(&author)
                    .await
                    .map_err(|err| {
                        generate_service_internal_error(
                            "Failed to fetch user from repository \
                            while parsing article fetch service query",
                            err,
                        )
                    })?
                    .map(|user| Some(ArticleQueryType::Author(user.id())))
                {
                    None => return Err(SamambaiaError::resource_not_found_err()),
                    Some(author_query) => {
                        if let Some(query) = author_query {
                            parsed_queries.push(query)
                        }
                    }
                }
            }
            FetchArticleQuery::Tags(tags_ids) => {
                let mut tags_ids = tags_ids.into_iter().map(ArticleQueryType::Tag).collect();
                parsed_queries.append(&mut tags_ids);
            }
        };
    }

    Ok(parsed_queries)
}
