use crate::domain::domain_entities::team_user::TeamUser;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::{LOG_SEP, R_EOL};
use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::team_user_repository::TeamUserRepositoryTrait;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::verify_role_has_permission;

pub struct UpdateTeamUserParams {
    pub staff_role: Role,
    pub team_user_id: Uuid,
    pub team_role_id: Option<Uuid>,
    pub nickname: Option<String>,
    pub user_function: Option<String>,
    pub twitter: Option<Option<String>>,
    pub discord: Option<Option<String>>,
}

pub struct UpdateTeamUserService<TeamUserRepository: TeamUserRepositoryTrait> {
    team_user_repository: Box<TeamUserRepository>,
}

impl<TeamUserRepository: TeamUserRepositoryTrait> UpdateTeamUserService<TeamUserRepository> {
    pub fn new(team_user_repository: Box<TeamUserRepository>) -> Self {
        UpdateTeamUserService {
            team_user_repository,
        }
    }

    pub async fn exec(
        &self,
        params: UpdateTeamUserParams,
    ) -> Result<TeamUser, Box<dyn DomainErrorTrait>> {
        // verifying staff/user can perform this action
        let user_can_update_team_user = verify_role_has_permission(
            &params.staff_role,
            crate::util::RolePermissions::UpdateTeamUser,
        );

        if !user_can_update_team_user {
            return Err(Box::new(UnauthorizedError::new()));
        }

        // fetching team user from database
        let team_user_on_db = self
            .team_user_repository
            .find_by_id(params.team_user_id)
            .await;

        if team_user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Update Article Service, while finding article by id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                team_user_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let team_user_on_db = team_user_on_db.unwrap();

        if team_user_on_db.is_none() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let mut team_user = team_user_on_db.unwrap();

        // updating the team user properties
        if params.team_role_id.is_some() {
            team_user.set_team_role_id(params.team_role_id.unwrap());
        }

        if params.discord.is_some() {
            team_user.set_discord(params.discord.unwrap());
        }

        if params.twitter.is_some() {
            team_user.set_twitter(params.twitter.unwrap());
        }

        if params.nickname.is_some() {
            team_user.set_nickname(params.nickname.unwrap());
        }

        if params.user_function.is_some() {
            team_user.set_user_function(params.user_function.unwrap());
        }

        // saving the changes
        let result = self.team_user_repository.save(team_user).await;

        if result.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Update Article Service, while finding article by id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                result.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        Ok(result.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio;

    use crate::domain::{
        domain_entities::team_role::TeamRole,
        repositories::team_user_repository::MockTeamUserRepositoryTrait,
    };

    #[tokio::test]
    async fn test() {
        // instantiating needed entities
        let team_role = TeamRole::new("CEO".into(), "Make damn shits".into());
        let team_user = TeamUser::new(
            "Floricultor".into(),
            "Bla bla bla".into(),
            None,
            None,
            team_role.id(),
        );

        // mocking the database
        let team_user_db: Arc<Mutex<Vec<TeamUser>>> = Arc::new(Mutex::new(Vec::new()));
        team_user_db.lock().unwrap().push(team_user.clone());

        // mocking the repository
        let mut mocked_team_user_repository = MockTeamUserRepositoryTrait::new();

        let db_clone = Arc::clone(&team_user_db);
        mocked_team_user_repository
            .expect_find_by_id()
            .returning(move |id| {
                for team_user in db_clone.lock().unwrap().iter() {
                    if team_user.id().eq(&id) {
                        return Ok(Some(team_user.clone()));
                    }
                }

                Ok(None)
            });

        let db_clone = Arc::clone(&team_user_db);
        mocked_team_user_repository
            .expect_save()
            .returning(move |param_team_user| {
                let mut index = None;

                for (i, tu) in db_clone.lock().unwrap().iter().enumerate() {
                    if tu.id().eq(&param_team_user.id()) {
                        index = Some(i);
                    }
                }

                db_clone.lock().unwrap()[index.unwrap()] = param_team_user.clone();
                Ok(param_team_user)
            });

        // testing
        let sut = UpdateTeamUserService::new(Box::new(mocked_team_user_repository));

        let res = sut
            .exec(UpdateTeamUserParams {
                discord: Some(Some("kaiofelps".into())),
                twitter: None,
                nickname: None,
                user_function: None,
                staff_role: Role::Ceo,
                team_role_id: None,
                team_user_id: team_user.id(),
            })
            .await;

        assert!(res.is_ok());

        assert_eq!(
            "kaiofelps".to_string(),
            team_user_db.lock().unwrap()[0].discord().unwrap()
        );
    }
}
