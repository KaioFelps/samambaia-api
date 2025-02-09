use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::team_user::TeamUser;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::domain::repositories::team_user_repository::TeamUserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct CreateTeamUserParams {
    pub nickname: String,
    pub user_function: String,
    pub twitter: Option<String>,
    pub discord: Option<String>,
    pub team_role_id: Uuid,
    pub staff_role: Role,
}

pub struct CreateTeamUserService<TeamUserRepository, TeamRoleRepository>
where
    TeamUserRepository: TeamUserRepositoryTrait,
    TeamRoleRepository: TeamRoleRepositoryTrait,
{
    team_user_repository: TeamUserRepository,
    team_role_repository: TeamRoleRepository,
}

impl<TeamUserRepository: TeamUserRepositoryTrait, TeamRoleRepository: TeamRoleRepositoryTrait>
    CreateTeamUserService<TeamUserRepository, TeamRoleRepository>
{
    pub fn new(
        team_user_repository: TeamUserRepository,
        team_role_repository: TeamRoleRepository,
    ) -> Self {
        CreateTeamUserService {
            team_user_repository,
            team_role_repository,
        }
    }

    pub async fn exec(&self, params: CreateTeamUserParams) -> Result<TeamUser, SamambaiaError> {
        let team_user = TeamUser::new(
            params.nickname,
            params.user_function,
            params.twitter,
            params.discord,
            params.team_role_id,
        );

        let staff_can_add_team_user =
            verify_role_has_permission(&params.staff_role, RolePermissions::CreateTeamUser);

        if !staff_can_add_team_user {
            return Err(SamambaiaError::unauthorized_err());
        }

        let role_on_db = self
            .team_role_repository
            .find_by_id(params.team_role_id)
            .await.map_err(|err| {
                generate_service_internal_error("Error occurred on Create Team User Service, while finding team role by Id on the database", err)
            })?;

        if role_on_db.is_none() {
            return Err(SamambaiaError::bad_request_err());
        }

        self.team_user_repository
            .create(team_user)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Create Team User Service, while persisting on the database",
                    err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use http::StatusCode;
    use tokio;

    use super::*;
    use crate::domain::domain_entities::team_role::TeamRole;
    use crate::domain::repositories::team_role_repository::MockTeamRoleRepositoryTrait;
    use crate::domain::repositories::team_user_repository::MockTeamUserRepositoryTrait;

    #[tokio::test]
    async fn test() {
        // mocking the enities
        let team_role = TeamRole::new("Ceo".into(), "Do some shits".into());

        // mocking the databases
        let team_role_database: Arc<Mutex<Vec<TeamRole>>> = Arc::new(Mutex::new(Vec::new()));
        let team_user_database: Arc<Mutex<Vec<TeamUser>>> = Arc::new(Mutex::new(Vec::new()));

        team_role_database.lock().unwrap().push(team_role.clone());

        // mocking the repositories
        let mut mocked_team_role_repo = MockTeamRoleRepositoryTrait::new();
        let mut mocked_team_user_repo = MockTeamUserRepositoryTrait::new();

        // mocking the repositories used methods
        let db = Arc::clone(&team_role_database);
        mocked_team_role_repo
            .expect_find_by_id()
            .returning(move |id| {
                for role in db.lock().unwrap().iter() {
                    if role.id().eq(&id) {
                        return Ok(Some(role.clone()));
                    }
                }

                Ok(None)
            });

        let db = Arc::clone(&team_user_database);
        mocked_team_user_repo
            .expect_create()
            .returning(move |user| {
                db.lock().unwrap().push(user.clone());
                Ok(user)
            });

        // testing
        let sut = CreateTeamUserService::new(mocked_team_user_repo, mocked_team_role_repo);

        let result = sut
            .exec(CreateTeamUserParams {
                nickname: "Floricultor".into(),
                user_function: "Do he shits a Ceo does".into(),
                discord: None,
                twitter: None,
                team_role_id: team_role.id(),
                staff_role: Role::Coord,
            })
            .await;

        // Coords are not allowed to manage team users, only admin above.
        assert_eq!(result.unwrap_err().get_code(), StatusCode::UNAUTHORIZED);

        let result = sut
            .exec(CreateTeamUserParams {
                nickname: "Floricultor".into(),
                user_function: "Do he shits a Ceo does".into(),
                discord: None,
                twitter: None,
                team_role_id: Uuid::new_v4(),
                staff_role: Role::Admin,
            })
            .await;

        // If team role id points to a non-existing team role, it throws a bad request error.
        assert_eq!(result.unwrap_err().get_code(), StatusCode::BAD_REQUEST);

        let result = sut
            .exec(CreateTeamUserParams {
                nickname: "Floricultor".into(),
                user_function: "Do he shits a Ceo does".into(),
                discord: None,
                twitter: None,
                team_role_id: team_role.id(),
                staff_role: Role::Admin,
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(team_user_database.lock().unwrap()[0], result.unwrap());
    }
}
