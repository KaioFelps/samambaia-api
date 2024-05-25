use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::verify_role_has_permission;
use crate::util::RolePermissions::DeleteTeamRole;

use crate::{R_EOL, LOG_SEP};

pub struct DeleteTeamRoleParams {
    pub staff_role: Role,
    pub team_role_id: Uuid,
}

pub struct DeleteTeamRoleService<TeamRoleRepository: TeamRoleRepositoryTrait> {
    team_role_repository: Box<TeamRoleRepository>
}

impl<TeamRoleRepository: TeamRoleRepositoryTrait> DeleteTeamRoleService<TeamRoleRepository> {
    pub fn new(team_role_repository: Box<TeamRoleRepository>) -> Self {
        DeleteTeamRoleService {
            team_role_repository
        }
    }

    pub async fn exec(&self, params: DeleteTeamRoleParams) -> Result<(), Box<dyn DomainErrorTrait>> {
        let staff_can_delete = verify_role_has_permission(&params.staff_role, DeleteTeamRole);

        if !staff_can_delete {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let team_role_on_db = self.team_role_repository
        .find_by_id(params.team_role_id)
        .await;

        if team_role_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Team Role Service, while fetching the team role from the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                team_role_on_db.unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let team_role_on_db = team_role_on_db.unwrap();

        if team_role_on_db.is_none() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let team_role = team_role_on_db.unwrap();

        let result = self.team_role_repository.delete(team_role).await;

        match result {
            Err(err) => {
                error!(
                    "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Team Role Service, while deleting the team role from the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                    err
                );
    
                Err(Box::new(InternalError::new()))
            },
            Ok(_) => Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio;
    
    use crate::domain::domain_entities::team_role::TeamRole;
    use crate::domain::repositories::team_role_repository::MockTeamRoleRepositoryTrait;

    #[tokio::test]
    async fn test() {
        let team_role_db: Arc<Mutex<Vec<TeamRole>>>
        = Arc::new(Mutex::new(Vec::new()));

        let tr = TeamRole::new("CEO".into(), "Description".into());

        team_role_db.lock().unwrap().push(tr.clone());

        // mocking the repository
        let mut mocked_team_role_repository = MockTeamRoleRepositoryTrait::new();

        let tr_db_clone = Arc::clone(&team_role_db);
        mocked_team_role_repository
        .expect_find_by_id()
        .returning(move |id| {
            for tr in tr_db_clone.lock().unwrap().iter() {
                if tr.id().eq(&id) {
                    return Ok(Some(tr.clone()));
                }
            }

            Ok(None)
        });

        let tr_db_clone = Arc::clone(&team_role_db);
        mocked_team_role_repository
        .expect_delete()
        .returning(move |tr| {
            let mut to_be_removed_index = None;

            for (index, db_tr) in tr_db_clone.lock().unwrap().iter().enumerate() {
                if db_tr.id().eq(&tr.id()) {
                    to_be_removed_index = Some(index);
                }
            }

            tr_db_clone.lock().unwrap().remove(to_be_removed_index.unwrap());

            Ok(())
        });

        let sut = DeleteTeamRoleService::new(Box::new(mocked_team_role_repository));

        let failing_res = sut.exec(DeleteTeamRoleParams {
            staff_role: Role::Principal,
            team_role_id: tr.id(),
        }).await;

        assert!(failing_res.is_err());

        let res = sut.exec(DeleteTeamRoleParams {
            staff_role: Role::Ceo,
            team_role_id: tr.id(),
        }).await;
        
        assert!(res.is_ok());
        assert_eq!(0, team_role_db.lock().unwrap().len());
    }
}
