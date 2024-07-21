use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::{generate_service_internal_error, RolePermissions, verify_role_has_permission};
use crate::errors::error::DomainErrorTrait;

pub struct CreateArticleTagParams {
    value: String,
    user_role: Role,
}

pub struct CreateArticleTagService<ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_tag_repository: ArticleTagRepository
}

impl <ArticleTagRepository: ArticleTagRepositoryTrait>CreateArticleTagService<ArticleTagRepository> {
    pub fn new(article_tag_repository: ArticleTagRepository) -> Self {
        CreateArticleTagService {
            article_tag_repository
        }
    }

    pub async fn exec(&self, params: CreateArticleTagParams) -> Result<ArticleTag, Box<dyn DomainErrorTrait>> {
        let user_can_create_tag = verify_role_has_permission(&params.user_role, RolePermissions::CreateArticleTag);

        if !user_can_create_tag {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let draft_tag = DraftArticleTag::new(params.value);
        let tag = self.article_tag_repository.create(draft_tag).await;

        if tag.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred in Article Tag Service while creating the tag in the database".into(),
                &tag.unwrap_err(),
            ));
        }

        let tag = tag.unwrap();
        Ok(tag)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::role::Role;
    use crate::domain::services::create_article_tag_service::CreateArticleTagParams;
    use crate::tests::repositories::article_tag_repository::get_article_tag_repository;

    #[tokio::test]
    async fn test_if_user_can_create_tag() {
        let (tag_db, tag_repository) = get_article_tag_repository();
        let sut = super::CreateArticleTagService::new(tag_repository);

        let result = sut.exec(CreateArticleTagParams { value: "Foo".into(), user_role: Role::Principal }).await;
        assert_eq!(tag_db.lock().unwrap().len(), 1);
        assert!(result.is_ok(), "Principal-role users should be able to create a new article tag.");
    }

    #[tokio::test]
    async fn test_if_unauthorized_user_cannot_create_tag() {
        let (db, tag_repository) = get_article_tag_repository();
        let sut = super::CreateArticleTagService::new(tag_repository);

        let result = sut.exec(CreateArticleTagParams { value: "Bar".into(), user_role: Role::Admin }).await;
        assert!(result.is_err());
        assert_eq!(db.lock().unwrap().len(), 0);
    }
}
