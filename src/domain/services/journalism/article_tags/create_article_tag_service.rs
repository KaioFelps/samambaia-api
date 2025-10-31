use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{RolePermissions, generate_service_internal_error, verify_role_has_permission};

pub struct CreateArticleTagParams {
    pub value: String,
    pub user_role: Role,
}

pub struct CreateArticleTagService<ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_tag_repository: ArticleTagRepository,
}

impl<ArticleTagRepository: ArticleTagRepositoryTrait>
    CreateArticleTagService<ArticleTagRepository>
{
    pub fn new(article_tag_repository: ArticleTagRepository) -> Self {
        CreateArticleTagService {
            article_tag_repository,
        }
    }

    pub async fn exec(&self, params: CreateArticleTagParams) -> Result<ArticleTag, SamambaiaError> {
        let user_can_create_tag =
            verify_role_has_permission(&params.user_role, RolePermissions::CreateArticleTag);

        if !user_can_create_tag {
            return Err(SamambaiaError::unauthorized_err());
        }

        let draft_tag = DraftArticleTag::new(params.value);
        self.article_tag_repository
            .create(draft_tag)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Article Tag Service while creating the tag in the database",
                    err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::role::Role;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;

    #[tokio::test]
    async fn test_if_user_can_create_tag() {
        let tag_repository = InMemoryArticleTagRepository::default();
        let sut = super::CreateArticleTagService::new(tag_repository.clone());

        let result = sut
            .exec(super::CreateArticleTagParams {
                value: "Foo".into(),
                user_role: Role::Principal,
            })
            .await;
        assert_eq!(1, tag_repository.tag_db.lock().unwrap().len());
        assert!(
            result.is_ok(),
            "Principal-role users should be able to create a new article tag."
        );
    }

    #[tokio::test]
    async fn test_if_unauthorized_user_cannot_create_tag() {
        let tag_repository = InMemoryArticleTagRepository::default();
        let sut = super::CreateArticleTagService::new(tag_repository.clone());

        let result = sut
            .exec(super::CreateArticleTagParams {
                value: "Bar".into(),
                user_role: Role::Admin,
            })
            .await;
        assert!(result.is_err());
        assert_eq!(0, tag_repository.tag_db.lock().unwrap().len());
    }
}
