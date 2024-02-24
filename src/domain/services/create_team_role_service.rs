use std::error::Error;
use log::error;
use uuid::Uuid;
use crate::{LOG_SEP, R_EOL};

use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::util::{verify_role_has_permission, RolePermissions};

pub struct CreateTeamRoleParams {
    pub title: String,
    pub description: String,
    pub staff_id: Uuid,
}

pub struct CreateTeamRoleService<
TeamRoleRepository: TeamRoleRepositoryTrait,
UserRepository: UserRepositoryTrait
> {
    team_role_repository: Box<TeamRoleRepository>,
    user_repository: Box<UserRepository>
}

impl<
    TeamRoleRepository: TeamRoleRepositoryTrait,
    UserRepository: UserRepositoryTrait
> CreateTeamRoleService<TeamRoleRepository, UserRepository> {
    pub fn new(
        team_role_repository: Box<TeamRoleRepository>,
        user_repository: Box<UserRepository>
    ) -> Self {
        CreateTeamRoleService {
            team_role_repository,
            user_repository
        }
    }

    pub async fn exec(&self, params: CreateTeamRoleParams) -> Result<TeamRole, Box<dyn Error>> {
        let CreateTeamRoleParams { title, description, staff_id } = params;

        let user_on_db = self.user_repository.find_by_id(&staff_id).await;

        if user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Team Role Service, while finding staff on the database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user_on_db.unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let user_on_db = user_on_db.unwrap();

        if user_on_db.is_none() { return Err(Box::new(UnauthorizedError::new())) }

        let user_on_db = user_on_db.unwrap();

        let user_can_create_team_role = verify_role_has_permission(&user_on_db.role().unwrap(), RolePermissions::CreateNewTeamRole);

        if !user_can_create_team_role { return Err(Box::new(UnauthorizedError::new())) }

        let team_role = TeamRole::new(title, description);

        let result = self.team_role_repository.create(team_role).await;

        match result {
            Err(err) => {
                error!(
                    "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Team Role Service, while persisting on the database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                    err
                );

                Err(Box::new(InternalError::new()))
            },
            Ok(team_role) => {
                Ok(team_role)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::{team_role_repository::MockTeamRoleRepositoryTrait, user_repository::MockUserRepositoryTrait};
    use crate::domain::domain_entities::user::User;

    use super::*;
    use tokio;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        // populating
        let admin_user = User::new("Salem".into(), "123".into(), Some(Role::Admin));
        let principal_user = User::new("Flori".into(), "123".into(), Some(Role::Principal));

        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
        let team_role_db: Arc<Mutex<Vec<TeamRole>>> = Arc::new(Mutex::new(Vec::new()));

        user_db.lock().unwrap().push(admin_user.clone());
        user_db.lock().unwrap().push(principal_user.clone());

        // mocking the repositories
        let mut mocked_user_repo = MockUserRepositoryTrait::new();
        let mut mocked_team_role_repo = MockTeamRoleRepositoryTrait::new();

        let arc_user_db = Arc::clone(&user_db);
        mocked_user_repo
        .expect_find_by_id()
        .returning(move |id| {
            for user in arc_user_db.lock().unwrap().iter() {
                if user.id().eq(id) {
                    return Ok(Some(user.clone()));
                }
            }

            Ok(None)
        });

        let arc_team_role_db = Arc::clone(&team_role_db);
        mocked_team_role_repo
        .expect_create()
        .returning(move |team_role| {
            arc_team_role_db.lock().unwrap().push(team_role.clone());

            Ok(team_role)
        });

        // testing
        let sut = CreateTeamRoleService::new(Box::new(mocked_team_role_repo), Box::new(mocked_user_repo));

        let response = sut.exec(CreateTeamRoleParams {
            title: "Editor-chefe".into(),
            description: "Responsável por supervisionar a edição e aprovar as notícias.".into(),
            staff_id: admin_user.id()
        }).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().to_string(), UnauthorizedError::new().to_string());

        let response = sut.exec(CreateTeamRoleParams {
            title: "Editor-chefe".into(),
            description: "Responsável por supervisionar a edição e aprovar as notícias.".into(),
            staff_id: principal_user.id()
        }).await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), team_role_db.lock().unwrap()[0]);
    }
}
