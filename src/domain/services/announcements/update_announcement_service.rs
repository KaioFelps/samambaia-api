use uuid::Uuid;

use crate::{
    domain::{
        domain_entities::announcement::Announcement,
        repositories::{
            announcements_repository::AnnouncementRepositoryTrait,
            user_repository::UserRepositoryTrait,
        },
    },
    error::DomainError,
    util::{generate_service_internal_error, verify_role_has_permission, RolePermissions},
};

pub struct UpdateAnnouncementParams {
    pub user_id: Uuid,
    pub announcement_id: Uuid,
    pub url: Option<String>,
    pub image: Option<String>,
    pub external: Option<bool>,
    pub description: Option<String>,
}

pub struct UpdateAnnouncementService<UR, AR>
where
    UR: UserRepositoryTrait,
    AR: AnnouncementRepositoryTrait,
{
    users_repository: Box<UR>,
    announcements_repository: Box<AR>,
}

impl<UR, AR> UpdateAnnouncementService<UR, AR>
where
    UR: UserRepositoryTrait,
    AR: AnnouncementRepositoryTrait,
{
    pub fn new(users_repository: Box<UR>, announcements_repository: Box<AR>) -> Self {
        UpdateAnnouncementService {
            users_repository,
            announcements_repository,
        }
    }

    pub async fn exec(
        &self,
        params: UpdateAnnouncementParams,
    ) -> Result<Announcement, DomainError> {
        let user = self
            .users_repository
            .find_by_id(&params.user_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on fetching user from database in Update Announcement Service",
                    err,
                )
            })?;

        let user_is_authorized = user.is_some_and(|user| {
            verify_role_has_permission(
                user.role().as_ref().unwrap(),
                RolePermissions::UpdateAnnouncement,
            )
        });

        if !user_is_authorized {
            return Err(DomainError::unauthorized_err());
        }

        let mut announcement = match self
            .announcements_repository
            .find_by_id(&params.announcement_id)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on fetchin announcement from database in Update Announocement Service",
                err
            ))?
        {
            None => return Err(DomainError::resource_not_found_err().with_message("Announcement not found.")),
            Some(announcement) => announcement,
        };

        if let Some(url) = params.url {
            announcement.set_url(url);
        }

        if let Some(image) = params.image {
            announcement.set_image(image);
        }

        if let Some(description) = params.description {
            announcement.set_description(description);
        }

        if let Some(external) = params.external {
            announcement.set_external(external);
        }

        let id = *announcement.id();

        self.announcements_repository
            .save(announcement)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    &format!("Error occurred on saving changes of announcement with id {id} in the database"),
                    err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use super::UpdateAnnouncementParams;
    use crate::{
        domain::domain_entities::{announcement::Announcement, role::Role, user::User},
        error::DomainError,
        tests::repositories::{
            announcements_repository::get_announcements_repository,
            users_repository::get_user_repository,
        },
    };

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_update_announcement_service() {
        let (users_db, users_repository) = get_user_repository();
        let (announcements_db, announcements_repository) = get_announcements_repository();

        let unauthorized_user = User::new("Foo".into(), "123".into(), Some(Role::Admin));
        let authorized_user = User::new("Bar".into(), "456".into(), Some(Role::Principal));

        let mut users_db_lock = users_db.lock().unwrap();
        users_db_lock.push(authorized_user.clone());
        users_db_lock.push(unauthorized_user.clone());

        drop(users_db_lock);

        let announcement = Announcement::new(
            "www.google.com".into(),
            "https://i.imgur.com".into(),
            false,
            authorized_user.id(),
            "Some description...".into(),
        );

        announcements_db.lock().unwrap().push(announcement.clone());

        let service = super::UpdateAnnouncementService::new(
            Box::new(users_repository),
            Box::new(announcements_repository),
        );

        let unauthorized_result = service
            .exec(UpdateAnnouncementParams {
                announcement_id: *announcement.id(),
                user_id: unauthorized_user.id(),
                description: None,
                external: Some(true),
                image: None,
                url: None,
            })
            .await;

        assert!(unauthorized_result.is_err());
        assert!(matches!(
            unauthorized_result.unwrap_err(),
            DomainError::Unauthorized(_)
        ));
    }
}
