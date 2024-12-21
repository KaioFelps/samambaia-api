use uuid::Uuid;

use crate::{
    domain::repositories::{
        announcements_repository::AnnouncementRepositoryTrait, user_repository::UserRepositoryTrait,
    },
    error::DomainError,
    util::{generate_service_internal_error, verify_role_has_permission, RolePermissions},
};

pub struct DeleteAnnouncementParams<'a> {
    user_id: &'a Uuid,
    announcement_id: &'a Uuid,
}

pub struct DeleteAnnouncementService<UR, AR>
where
    UR: UserRepositoryTrait,
    AR: AnnouncementRepositoryTrait,
{
    users_repository: Box<UR>,
    announcements_repository: Box<AR>,
}

impl<UR, AR> DeleteAnnouncementService<UR, AR>
where
    UR: UserRepositoryTrait,
    AR: AnnouncementRepositoryTrait,
{
    pub fn new(users_repository: Box<UR>, announcements_repository: Box<AR>) -> Self {
        DeleteAnnouncementService {
            users_repository,
            announcements_repository,
        }
    }

    pub async fn exec(&self, params: DeleteAnnouncementParams<'_>) -> Result<(), DomainError> {
        if !self
            .users_repository
            .find_by_id(params.user_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Delete Announcement Service, on fetching user from database",
                    err,
                )
            })?
            .is_some_and(|user| {
                verify_role_has_permission(
                    user.role().as_ref().unwrap(),
                    RolePermissions::DeleteAnnouncement,
                )
            })
        {
            return Err(DomainError::unauthorized_err());
        }

        self.announcements_repository
            .delete(params.announcement_id)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred in Delete Announcement Service, on deleting the announcement from the database",
                err
            ))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::domain_entities::{announcement::Announcement, role::Role, user::User},
        tests::repositories::{
            announcements_repository::get_announcements_repository,
            users_repository::get_user_repository,
        },
    };

    #[tokio::test]
    async fn test_delete_announcement_service() {
        let (user_db, users_repository) = get_user_repository();
        let (announcement_db, announcements_repository) = get_announcements_repository();

        let unauthorized_user = User::new("John Doe".into(), "123".into(), Some(Role::Admin));
        let authorized_user = User::new("John Foo".into(), "123".into(), Some(Role::Principal));
        let announcement = Announcement::new(
            "www.google.com".into(),
            "imagelink.com".into(),
            false,
            unauthorized_user.id(),
            "description".into(),
        );

        user_db.lock().unwrap().push(unauthorized_user.clone());
        user_db.lock().unwrap().push(authorized_user.clone());
        announcement_db.lock().unwrap().push(announcement.clone());

        let service = super::DeleteAnnouncementService::new(
            Box::new(users_repository),
            Box::new(announcements_repository),
        );

        let failure_result = service
            .exec(super::DeleteAnnouncementParams {
                announcement_id: announcement.id(),
                user_id: &unauthorized_user.id(),
            })
            .await;

        assert!(failure_result.is_err());
        assert!(!announcement_db.lock().unwrap().is_empty());

        let success_result = service
            .exec(super::DeleteAnnouncementParams {
                announcement_id: announcement.id(),
                user_id: &authorized_user.id(),
            })
            .await;

        assert!(success_result.is_ok());
        assert!(announcement_db.lock().unwrap().is_empty());
    }
}
