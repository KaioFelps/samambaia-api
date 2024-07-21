use crate::core::pagination::{DEFAULT_PER_PAGE, PaginationParameters, PaginationResponse};
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::repositories::article_tag_repository::{ArticleTagQueryType, ArticleTagRepositoryTrait, FindManyArticleTagsResponse};
use crate::errors::error::DomainErrorTrait;
use crate::util::generate_service_internal_error;

pub struct FetchManyArticleTagsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<String>,
}

pub struct FetchManyArticleTagsResponse {
    pub data: Vec<ArticleTag>,
    pub pagination: PaginationResponse
}

pub struct FetchManyArticleTagsService<ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_tag_repository: ArticleTagRepository
}

impl<ArticleTagRepository: ArticleTagRepositoryTrait> FetchManyArticleTagsService<ArticleTagRepository> {
    pub fn new(article_tag_repository: ArticleTagRepository) -> Self {
        FetchManyArticleTagsService {
            article_tag_repository
        }
    }

    pub async fn exec(&self, params: FetchManyArticleTagsParams) -> Result<FetchManyArticleTagsResponse, Box<dyn DomainErrorTrait>> {
        let per_page = params.per_page.unwrap_or(DEFAULT_PER_PAGE as u32);
        let page = match params.page {
            None => 1,
            Some(page) => if page <= 0 { 1 } else { page }
        };

        let query = match params.query {
            None => None,
            Some(query) => Some(ArticleTagQueryType::Value(query))
        };

        let response = self.article_tag_repository.find_many(PaginationParameters {
            page,
            items_per_page: per_page,
            query,
        }).await;

        if let Err(error) = response {
            return Err(generate_service_internal_error(
                "Error occurred in Fetch Many Article Tags Service, while finding many tags from the database".into(),
                &error
            ));
        }

        let FindManyArticleTagsResponse (tags, total_items) = response.unwrap();

        Ok(FetchManyArticleTagsResponse {
            data: tags,
            pagination: PaginationResponse {
                current_page: page,
                total_items,
                total_pages: (total_items as f64 / per_page as f64).ceil() as u32
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::domain::services::fetch_many_article_tags_service::FetchManyArticleTagsParams;
    use crate::tests::repositories::article_tag_repository::get_article_tag_repository;

    #[tokio::test]
    async fn it_can_fetch_many_article_tags() {
        let (tag_db, tag_repository) = get_article_tag_repository();

        tag_db.lock().unwrap().push(ArticleTag::new_from_existing(1, "Bar".into()));
        tag_db.lock().unwrap().push(ArticleTag::new_from_existing(2, "Foo".into()));

        let sut = super::FetchManyArticleTagsService::new(tag_repository);

        let result = sut.exec(FetchManyArticleTagsParams {
            query: Some("foo".into()),
            page: None,
            per_page: None,
        }).await;

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(1, result.pagination.total_items, "Expected total items to be 1.");
        assert_eq!(1, result.pagination.total_pages, "Expected total pages to be 1.");
        assert_eq!(1, result.data.len(), "Expected response data to be length 1.");
        assert_eq!(&"Foo".to_string(), result.data[0].value(), "Expected data list unique tag to have value 'Foo'.");
    }
}
