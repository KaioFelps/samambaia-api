use crate::domain::domain_entities::free_badge::FreeBadge;
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::free_badge_repository::FreeBadgeRepositoryTrait;
use crate::errors::bad_request_error::BadRequestError;
use crate::errors::error::DomainErrorTrait;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct UpdateFreeBadgeParams {
    pub free_badge_id: Uuid,
    pub user_role: Role,
    pub code: Option<String>,
    pub image: Option<String>,
    pub link: Option<String>,
    pub link_is_external: Option<bool>,
    pub available_until: Option<Option<NaiveDateTime>>,
}

pub struct UpdateFreeBadgeService<FreeBadgeRepository: FreeBadgeRepositoryTrait> {
    free_badge_repository: FreeBadgeRepository,
}

impl<FreeBadgeRepository: FreeBadgeRepositoryTrait> UpdateFreeBadgeService<FreeBadgeRepository> {
    pub fn new(free_badge_repository: FreeBadgeRepository) -> Self {
        UpdateFreeBadgeService {
            free_badge_repository,
        }
    }

    pub async fn exec(
        &self,
        params: UpdateFreeBadgeParams,
    ) -> Result<FreeBadge, Box<dyn DomainErrorTrait>> {
        let user_can_edit_free_badge =
            verify_role_has_permission(&params.user_role, RolePermissions::UpdateFreeBadge);

        if !user_can_edit_free_badge {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let mut free_badge = match self
            .free_badge_repository
            .find_by_id(params.free_badge_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Update Free Badge Service, on finding the free badge by the given id",
                    err,
                )
            })?
            {
                None =>return Err(Box::new(BadRequestError::new())),
                Some(badge) => badge,
            };

        if params.code.is_some() {
            free_badge.set_code(params.code.unwrap());
        }

        if params.image.is_some() {
            free_badge.set_image(params.image.unwrap());
        }

        if params.link.is_some() {
            free_badge.set_link(params.link.unwrap());
        }

        if params.link_is_external.is_some() {
            free_badge.set_link_is_external(params.link_is_external.unwrap());
        }

        if params.available_until.is_some() {
            free_badge.set_available_until(params.available_until.unwrap());
        }

        self.free_badge_repository
            .save(free_badge)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Update Free Badge Service on updating the free badge in the database",
                   err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::free_badge::FreeBadge;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::services::update_free_badge_service::UpdateFreeBadgeParams;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::free_badge_repository::get_free_badge_repository;

    #[tokio::test]
    async fn test_if_authorized_user_can_edit_free_badge() {
        let (badges_db, badge_repository) = get_free_badge_repository();
        let sut = super::UpdateFreeBadgeService::new(badge_repository);

        let free_badge = FreeBadge::new(
            "BR001".into(),
            "i.imgur.com".into(),
            "www.cosmic.com/news/a".into(),
            false,
            None,
        );
        badges_db.lock().unwrap().push(free_badge.clone());

        let badge_deadline = TimeHelper::now() + chrono::Days::new(5);

        let result = sut
            .exec(UpdateFreeBadgeParams {
                free_badge_id: free_badge.id(),
                user_role: Role::Writer,
                code: None,
                image: Some("habbo.com/imager/badges/BR001".into()),
                link: None,
                link_is_external: None,
                available_until: Some(Some(badge_deadline)),
            })
            .await;

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(badges_db.lock().unwrap()[0], result);
        assert_eq!(badge_deadline, result.available_until().unwrap());
        assert_eq!(&"BR001".to_string(), badges_db.lock().unwrap()[0].code());
        assert_eq!(
            &"habbo.com/imager/badges/BR001".to_string(),
            badges_db.lock().unwrap()[0].image()
        );
    }

    #[tokio::test]
    async fn test_if_non_auth_user_cannot_edit_free_badge() {
        let (badges_db, badge_repository) = get_free_badge_repository();
        let sut = super::UpdateFreeBadgeService::new(badge_repository);

        let free_badge = FreeBadge::new(
            "BR001".into(),
            "i.imgur.com".into(),
            "www.cosmic.com/news/a".into(),
            false,
            None,
        );
        badges_db.lock().unwrap().push(free_badge.clone());

        let badge_deadline = TimeHelper::now() + chrono::Days::new(5);

        let result = sut
            .exec(UpdateFreeBadgeParams {
                free_badge_id: free_badge.id(),
                user_role: Role::User,
                code: None,
                image: Some("habbo.com/imager/badges/BR001".into()),
                link: None,
                link_is_external: None,
                available_until: Some(Some(badge_deadline)),
            })
            .await;

        assert!(result.is_err());
    }
}
