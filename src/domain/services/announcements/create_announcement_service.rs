use uuid::Uuid;

use crate::{
    domain::{
        domain_entities::announcement::Announcement,
        repositories::{
            announcements_repository::AnnouncementRepositoryTrait,
            user_repository::UserRepositoryTrait,
        },
    },
    error::SamambaiaError,
    util::{generate_service_internal_error, verify_role_has_permission, RolePermissions},
};

pub struct CreateAnnouncementParams {
    pub url: String,
    pub image: String,
    pub external: bool,
    pub staff_id: Uuid,
    pub description: String,
}

pub struct CreateAnnouncementService<UR, AR>
where
    UR: UserRepositoryTrait,
    AR: AnnouncementRepositoryTrait,
{
    users_repository: UR,
    announcements_repository: AR,
}

impl<UR, AR> CreateAnnouncementService<UR, AR>
where
    UR: UserRepositoryTrait,
    AR: AnnouncementRepositoryTrait,
{
    pub fn new(users_repository: UR, announcements_repository: AR) -> Self {
        CreateAnnouncementService {
            users_repository,
            announcements_repository,
        }
    }

    pub async fn exec(
        &self,
        CreateAnnouncementParams {
            url,
            image,
            external,
            staff_id,
            description,
        }: CreateAnnouncementParams,
    ) -> Result<Announcement, SamambaiaError> {
        let staff = self
            .users_repository
            .find_by_id(&staff_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred at create announcement service, while fetching staff user from the database",
                    err
                )
            })?;

        if !staff
            .as_ref()
            .and_then(|staff| staff.role())
            .is_some_and(|role| {
                verify_role_has_permission(&role, RolePermissions::CreateAnnouncement)
            })
        {
            return Err(SamambaiaError::unauthorized_err());
        }

        let announcement = Announcement::new(url, image, external, staff_id, description);
        let announcement = self
            .announcements_repository
            .create(announcement)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on saving the announcement in the database",
                    err,
                )
            })?;

        Ok(announcement)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::domain_entities::{role::Role, user::User},
        error::SamambaiaError,
        tests::repositories::{
            announcements_repository::get_announcements_repository,
            users_repository::get_user_repository,
        },
    };

    #[tokio::test]
    async fn test_create_announcement_service() {
        let (announcements_db, announcements_repository) = get_announcements_repository();
        let (users_db, users_repository) = get_user_repository();

        let unauthorized_user = User::new("JohnDoe".into(), "foo".into(), Some(Role::User));
        let authorized_user = User::new("Carmin".into(), "123".into(), Some(Role::Principal));

        users_db.lock().unwrap().push(unauthorized_user.clone());
        users_db.lock().unwrap().push(authorized_user.clone());

        let service = super::CreateAnnouncementService {
            users_repository,
            announcements_repository,
        };

        let failure_call = service
            .exec(super::CreateAnnouncementParams {
                external: true,
                image: "imgur.com".into(),
                url: "www.google.com".into(),
                staff_id: unauthorized_user.id(),
                description: "Foo bar.".into(),
            })
            .await;

        assert!(failure_call.is_err());
        assert!(matches!(
            failure_call.unwrap_err(),
            SamambaiaError::Unauthorized(_)
        ));

        let successful_call = service
            .exec(super::CreateAnnouncementParams {
                external: true,
                image: "imgur.com".into(),
                url: "www.google.com".into(),
                staff_id: authorized_user.id(),
                description: "Foo bar.".into(),
            })
            .await;

        assert!(successful_call.is_ok());
        assert!(!announcements_db.lock().unwrap().is_empty());
    }
}
