use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::error::DomainError;
use crate::util::RolePermissions::DeleteTeamRole;
use crate::util::{generate_service_internal_error, verify_role_has_permission};

pub struct DeleteTeamRoleParams {
    pub staff_role: Role,
    pub team_role_id: Uuid,
}

pub struct DeleteTeamRoleService<TeamRoleRepository: TeamRoleRepositoryTrait> {
    team_role_repository: Box<TeamRoleRepository>,
}

impl<TeamRoleRepository: TeamRoleRepositoryTrait> DeleteTeamRoleService<TeamRoleRepository> {
    pub fn new(team_role_repository: Box<TeamRoleRepository>) -> Self {
        DeleteTeamRoleService {
            team_role_repository,
        }
    }

    pub async fn exec(&self, params: DeleteTeamRoleParams) -> Result<(), DomainError> {
        let staff_can_delete = verify_role_has_permission(&params.staff_role, DeleteTeamRole);

        if !staff_can_delete {
            return Err(DomainError::unauthorized_err());
        }

        let team_role_on_db = self
            .team_role_repository
            .find_by_id(params.team_role_id)
            .await.map_err(|err| generate_service_internal_error(
                "Error occurred on Delete Team Role Service, while fetching the team role from the database",
                err,
            ))?;

        if team_role_on_db.is_none() {
            return Err(DomainError::resource_not_found_err());
        }

        let team_role = team_role_on_db.unwrap();

        self.team_role_repository.delete(team_role)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Delete Team Role Service, while deleting the team role from the database",
                err,
            ))
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
        let team_role_db: Arc<Mutex<Vec<TeamRole>>> = Arc::new(Mutex::new(Vec::new()));

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

                tr_db_clone
                    .lock()
                    .unwrap()
                    .remove(to_be_removed_index.unwrap());

                Ok(())
            });

        let sut = DeleteTeamRoleService::new(Box::new(mocked_team_role_repository));

        let failing_res = sut
            .exec(DeleteTeamRoleParams {
                staff_role: Role::Principal,
                team_role_id: tr.id(),
            })
            .await;

        assert!(failing_res.is_err());

        let res = sut
            .exec(DeleteTeamRoleParams {
                staff_role: Role::Ceo,
                team_role_id: tr.id(),
            })
            .await;

        assert!(res.is_ok());
        assert_eq!(0, team_role_db.lock().unwrap().len());
    }
}
