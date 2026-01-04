use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub struct FindArticleTagByIdParams {
    pub tag_id: u32,
}

#[derive(Debug)]
pub struct FindArticleTagByIdResponse {
    pub tag: Option<ArticleTag>,
}

pub struct FindArticleTagByIdService<ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_tag_repository: ArticleTagRepository,
}

impl<ArticleTagRepository: ArticleTagRepositoryTrait>
    FindArticleTagByIdService<ArticleTagRepository>
{
    pub fn new(article_tag_repository: ArticleTagRepository) -> Self {
        FindArticleTagByIdService {
            article_tag_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FindArticleTagByIdParams,
    ) -> Result<FindArticleTagByIdResponse, SamambaiaError> {
        let tag = self
            .article_tag_repository
            .find_by_id(params.tag_id as i32)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Find Article Tag By ID service \
                    when fetching tag from the database",
                    err,
                )
            })?;

        Ok(FindArticleTagByIdResponse { tag })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;

    #[tokio::test]
    async fn it_should_find_existing_tags_by_ids() {
        let tag_repository = InMemoryArticleTagRepository::default();

        tag_repository
            .tag_db
            .lock()
            .unwrap()
            .push(ArticleTag::new_from_existing(1, "Bar".into()));

        tag_repository
            .tag_db
            .lock()
            .unwrap()
            .push(ArticleTag::new_from_existing(2, "Foo".into()));

        let sut = super::FindArticleTagByIdService::new(tag_repository);

        let result = sut
            .exec(super::FindArticleTagByIdParams { tag_id: 1 })
            .await;

        assert!(result.is_ok_and(|result| result.tag.is_some_and(|tag| tag.id() == 1)));
    }

    #[tokio::test]
    async fn it_should_return_none_if_there_is_no_article_tag_with_given_id() {
        let tag_repository = InMemoryArticleTagRepository::default();

        let sut = super::FindArticleTagByIdService::new(tag_repository);

        let result = sut
            .exec(super::FindArticleTagByIdParams { tag_id: 1 })
            .await;

        assert!(result.is_ok_and(|result| result.tag.is_none()));
    }
}
