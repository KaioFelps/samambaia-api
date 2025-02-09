use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::free_badge_repository::FreeBadgeRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct DeleteFreeBadgeParams {
    pub user_role: Role,
    pub free_badge_id: Uuid,
}

pub struct DeleteFreeBadgeService<FreeBadgeRepository: FreeBadgeRepositoryTrait> {
    free_badge_repository: FreeBadgeRepository,
}

impl<FreeBadgeRepository: FreeBadgeRepositoryTrait> DeleteFreeBadgeService<FreeBadgeRepository> {
    pub fn new(free_badge_repository: FreeBadgeRepository) -> Self {
        DeleteFreeBadgeService {
            free_badge_repository,
        }
    }

    pub async fn exec(&self, params: DeleteFreeBadgeParams) -> Result<(), SamambaiaError> {
        let user_can_delete_free_badge =
            verify_role_has_permission(&params.user_role, RolePermissions::DeleteFreeBadge);

        if !user_can_delete_free_badge {
            return Err(SamambaiaError::unauthorized_err());
        }

        let free_badge = self
            .free_badge_repository
            .find_by_id(params.free_badge_id)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred in Delete Free Badge Service, on finding free badge by id from the database",
                err,
            ))?;

        if free_badge.is_none() {
            return Err(SamambaiaError::resource_not_found_err());
        }

        let free_badge = free_badge.unwrap();

        self.free_badge_repository
            .delete(free_badge)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Delete Free Badge Service, on deleting free badge from database",
                   err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::free_badge::FreeBadge;
    use crate::domain::domain_entities::role::Role;
    use crate::tests::repositories::free_badge_repository::get_free_badge_repository;

    #[tokio::test]
    async fn test_if_allowed_user_can_delete_free_badge() {
        let (badges_db, badge_repository) = get_free_badge_repository();
        let sut = super::DeleteFreeBadgeService::new(badge_repository);

        let badge = FreeBadge::new(
            "PT001".to_string(),
            "image".to_string(),
            "link".to_string(),
            false,
            None,
        );
        let badge_id = badge.id();

        badges_db.lock().unwrap().push(badge);

        let result = sut
            .exec(super::DeleteFreeBadgeParams {
                free_badge_id: badge_id,
                user_role: Role::Writer,
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(badges_db.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_if_not_allowed_user_cannot_delete_free_badge() {
        let (badges_db, badge_repository) = get_free_badge_repository();
        let sut = super::DeleteFreeBadgeService::new(badge_repository);

        let badge = FreeBadge::new(
            "PT001".to_string(),
            "image".to_string(),
            "link".to_string(),
            false,
            None,
        );
        let badge_id = badge.id();

        badges_db.lock().unwrap().push(badge);

        let result = sut
            .exec(super::DeleteFreeBadgeParams {
                free_badge_id: badge_id,
                user_role: Role::User,
            })
            .await;

        assert!(result.is_err());
        assert_eq!(badges_db.lock().unwrap().len(), 1);
    }
}
