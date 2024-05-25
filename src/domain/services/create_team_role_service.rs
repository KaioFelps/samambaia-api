use log::error;
use crate::domain::domain_entities::role::Role;
use crate::errors::error::DomainErrorTrait;
use crate::{LOG_SEP, R_EOL};

use crate::errors::internal_error::InternalError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::util::{verify_role_has_permission, RolePermissions};

pub struct CreateTeamRoleParams {
    pub title: String,
    pub description: String,
    pub staff_role: Role
}

pub struct CreateTeamRoleService<TeamRoleRepository: TeamRoleRepositoryTrait> {
    team_role_repository: Box<TeamRoleRepository>,
}

impl<TeamRoleRepository: TeamRoleRepositoryTrait>
CreateTeamRoleService<TeamRoleRepository> {
    pub fn new(
        team_role_repository: Box<TeamRoleRepository>,
    ) -> Self {
        CreateTeamRoleService {
            team_role_repository,
        }
    }

    pub async fn exec(&self, params: CreateTeamRoleParams) -> Result<TeamRole, Box<dyn DomainErrorTrait>> {
        let CreateTeamRoleParams { title, description, staff_role } = params;

        let user_can_create_team_role = verify_role_has_permission(&staff_role, RolePermissions::CreateNewTeamRole);

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
    use crate::domain::repositories::team_role_repository::MockTeamRoleRepositoryTrait;

    use super::*;
    use tokio;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        // populating
        let team_role_db: Arc<Mutex<Vec<TeamRole>>> = Arc::new(Mutex::new(Vec::new()));

        // mocking the repositories
        let mut mocked_team_role_repo = MockTeamRoleRepositoryTrait::new();

        let arc_team_role_db = Arc::clone(&team_role_db);
        mocked_team_role_repo
        .expect_create()
        .returning(move |team_role| {
            arc_team_role_db.lock().unwrap().push(team_role.clone());

            Ok(team_role)
        });

        // testing
        let sut = CreateTeamRoleService::new(Box::new(mocked_team_role_repo));

        let response = sut.exec(CreateTeamRoleParams {
            title: "Editor-chefe".into(),
            description: "Responsável por supervisionar a edição e aprovar as notícias.".into(),
            staff_role: Role::Admin
        }).await;

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().to_string(), UnauthorizedError::new().to_string());

        let response = sut.exec(CreateTeamRoleParams {
            title: "Editor-chefe".into(),
            description: "Responsável por supervisionar a edição e aprovar as notícias.".into(),
            staff_role: Role::Principal
        }).await;

        assert!(response.is_ok());
        assert_eq!(response.unwrap(), team_role_db.lock().unwrap()[0]);
    }
}
