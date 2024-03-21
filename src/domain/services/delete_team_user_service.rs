use std::error::Error;
use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::team_user_repository::TeamUserRepositoryTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::verify_role_has_permission;
use crate::util::RolePermissions::DeleteTeamUser;

use crate::{R_EOL, LOG_SEP};

pub struct DeleteTeamUserParams {
    pub staff_role: Role,
    pub team_user_id: Uuid,
}

pub struct DeleteTeamUserService<TeamUserRepository: TeamUserRepositoryTrait> {
    team_user_repository: Box<TeamUserRepository>
}

impl<TeamUserRepository: TeamUserRepositoryTrait> DeleteTeamUserService<TeamUserRepository> {
    pub fn new(team_user_repository: Box<TeamUserRepository>) -> Self {
        DeleteTeamUserService {
            team_user_repository
        }
    }

    pub async fn exec(&self, params: DeleteTeamUserParams) -> Result<(), Box<dyn Error>> {
        let staff_can_delete = verify_role_has_permission(&params.staff_role, DeleteTeamUser);

        if !staff_can_delete {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let team_user_on_db = self.team_user_repository
        .find_by_id(params.team_user_id)
        .await;

        if team_user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Team User Service, while fetching the team user from the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                team_user_on_db.unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let team_user_on_db = team_user_on_db.unwrap();

        if team_user_on_db.is_none() {
            return Err(Box::new(ResourceNotFoundError::new()));
        }

        let team_user = team_user_on_db.unwrap();

        let result = self.team_user_repository.delete(team_user).await;

        match result {
            Err(err) => {
                error!(
                    "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Team User Service, while deleting the team user from the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
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
    use crate::domain::domain_entities::team_user::TeamUser;
    use crate::domain::repositories::team_user_repository::MockTeamUserRepositoryTrait;

    #[tokio::test]
    async fn test() {
        let team_user_db: Arc<Mutex<Vec<TeamUser>>> = Arc::new(Mutex::new(Vec::new()));

        let team_role = TeamRole::new("CEO".into(), "Anything".into());

        let team_user: TeamUser = TeamUser::new("Floricultor".into(), "blablabla".into(), None, None, team_role.id());

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

            tr_db_clone.lock().unwrap().remove(to_be_removed_index.unwrap());

            Ok(())
        });

        let sut = DeleteTeamUserService::new(Box::new(mocked_team_user_repository));

        let failing_res = sut.exec(DeleteTeamUserParams {
            staff_role: Role::Coord,
            team_user_id: team_user.id(),
        }).await;

        assert!(failing_res.is_err());

        let res = sut.exec(DeleteTeamUserParams {
            staff_role: Role::Admin,
            team_user_id: team_user.id(),
        }).await;
        
        assert!(res.is_ok());
        assert_eq!(0, team_user_db.lock().unwrap().len());
    }
}
