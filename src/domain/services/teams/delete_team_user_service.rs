use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::team_user_repository::TeamUserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::RolePermissions::DeleteTeamUser;
use crate::util::{generate_service_internal_error, verify_role_has_permission};

pub struct DeleteTeamUserParams {
    pub staff_role: Role,
    pub team_user_id: Uuid,
}

pub struct DeleteTeamUserService<TeamUserRepository: TeamUserRepositoryTrait> {
    team_user_repository: TeamUserRepository,
}

impl<TeamUserRepository: TeamUserRepositoryTrait> DeleteTeamUserService<TeamUserRepository> {
    pub fn new(team_user_repository: TeamUserRepository) -> Self {
        DeleteTeamUserService {
            team_user_repository,
        }
    }

    pub async fn exec(&self, params: DeleteTeamUserParams) -> Result<(), SamambaiaError> {
        let staff_can_delete = verify_role_has_permission(&params.staff_role, DeleteTeamUser);

        if !staff_can_delete {
            return Err(SamambaiaError::unauthorized_err());
        }

        let team_user_on_db = self
            .team_user_repository
            .find_by_id(params.team_user_id)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Delete Team User Service, while fetching the team user from the database",
                err
            ))?;

        if team_user_on_db.is_none() {
            return Err(SamambaiaError::resource_not_found_err());
        }

        let team_user = team_user_on_db.unwrap();

        self.team_user_repository.delete(team_user).await
        .map_err(|err| generate_service_internal_error(
            "Error occurred on Delete Team User Service, while deleting the team user from the database",
            err
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio;

    use crate::domain::domain_entities::team_role::TeamRole;
    use crate::domain::domain_entities::team_user::TeamUser;
    use crate::domain::repositories::team_user_repository::MockTeamUserRepositoryTrait;

    #[tokio::test]
    async fn test() {
        let team_user_db: Arc<Mutex<Vec<TeamUser>>> = Arc::new(Mutex::new(Vec::new()));

        let team_role = TeamRole::new("CEO".into(), "Anything".into());

        let team_user: TeamUser = TeamUser::new(
            "Floricultor".into(),
            "blablabla".into(),
            None,
            None,
            team_role.id(),
        );

        team_user_db.lock().unwrap().push(team_user.clone());

        // mocking the repository
        let mut mocked_team_user_repository = MockTeamUserRepositoryTrait::new();

        let tr_db_clone = Arc::clone(&team_user_db);
        mocked_team_user_repository
            .expect_find_by_id()
            .returning(move |id| {
                for tr in tr_db_clone.lock().unwrap().iter() {
                    if tr.id().eq(&id) {
                        return Ok(Some(tr.clone()));
                    }
                }

                Ok(None)
            });

        let tr_db_clone = Arc::clone(&team_user_db);
        mocked_team_user_repository
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

        let sut = DeleteTeamUserService::new(mocked_team_user_repository);

        let failing_res = sut
            .exec(DeleteTeamUserParams {
                staff_role: Role::Coord,
                team_user_id: team_user.id(),
            })
            .await;

        assert!(failing_res.is_err());

        let res = sut
            .exec(DeleteTeamUserParams {
                staff_role: Role::Admin,
                team_user_id: team_user.id(),
            })
            .await;

        assert!(res.is_ok());
        assert_eq!(0, team_user_db.lock().unwrap().len());
    }
}
