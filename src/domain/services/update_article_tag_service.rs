use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::errors::bad_request_error::BadRequestError;
use crate::errors::error::DomainErrorTrait;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::{generate_service_internal_error, RolePermissions, verify_role_has_permission};

pub struct UpdateArticleTagParams {
    user_role: Role,
    value: Option<String>,
    tag_id: i32,
}

pub struct UpdateArticleTagService<ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_tag_repository: ArticleTagRepository
}

impl<ArticleTagRepository: ArticleTagRepositoryTrait> UpdateArticleTagService<ArticleTagRepository> {
    pub fn new(article_tag_repository: ArticleTagRepository) -> Self {
        UpdateArticleTagService {
            article_tag_repository
        }
    }

    pub async fn exec(&self, params: UpdateArticleTagParams) -> Result<ArticleTag, Box<dyn DomainErrorTrait>> {
        let user_can_update_tag = verify_role_has_permission(&params.user_role, RolePermissions::UpdateArticleTag);

        if !user_can_update_tag {
            return Err(Box::new(UnauthorizedError::new()));
        }

        if params.value.is_none() {
            return Err(Box::new(BadRequestError::new_with_message(
                "Cannot perform an update if there is nothing to be updated.".into()
            )))
        }


        let tag_from_db = self.article_tag_repository.find_by_id(params.tag_id).await;

        if tag_from_db.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred in Update Article Service, while finding tag by id from database.".into(),
                &tag_from_db.unwrap_err()
            ));
        }

        let tag = tag_from_db.unwrap();

        if tag.is_none() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let mut tag = tag.unwrap();
        tag.set_value(params.value.unwrap());

        let result = self.article_tag_repository.save(tag).await;

        if result.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred in Update Article Service, while saving the updated tag.".into(),
                &result.unwrap_err()
            ));
        }

        let tag = result.unwrap();
        Ok(tag)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::services::update_article_tag_service::UpdateArticleTagParams;
    use crate::tests::repositories::article_tag_repository::get_article_tag_repository;

    #[tokio::test]
    async fn test_if_user_can_update_article_tag() {
        let (tag_db, tag_repository) = get_article_tag_repository();
        let sut = super::UpdateArticleTagService::new(tag_repository);

        let tag = ArticleTag::new_from_existing(1, "Foo".into());
        tag_db.lock().unwrap().push(tag);

        let result = sut.exec(UpdateArticleTagParams {
            value: Some("Bar".to_string()),
            user_role: Role::Principal,
            tag_id: 1,
        }).await;

        assert!(result.is_ok());
        assert_eq!(&"Bar".to_string(), tag_db.lock().unwrap()[0].value(), "Principal-role user should be able to update an article tag.");
    }

    #[tokio::test]
    async fn test_if_non_authorized_user_cannot_update_article_tag() {
        let (tag_db, tag_repository) = get_article_tag_repository();
        let sut = super::UpdateArticleTagService::new(tag_repository);

        let tag = ArticleTag::new_from_existing(1, "Foo".into());
        tag_db.lock().unwrap().push(tag);

        let result = sut.exec(UpdateArticleTagParams {
            value: Some("Bar".into()),
            user_role: Role::Admin,
            tag_id: 1,
        }).await;

        assert!(result.is_err(), "Only Principal-role or above users should be able to update an article tag.");
    }
}
