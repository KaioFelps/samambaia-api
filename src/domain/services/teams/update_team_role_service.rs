use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::TeamRoleRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission};
use uuid::Uuid;

pub struct UpdateTeamRoleParams {
    pub staff_role: Role,
    pub team_role_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub struct UpdateTeamRoleService<TeamRoleRepository: TeamRoleRepositoryTrait> {
    team_role_repository: TeamRoleRepository,
}

impl<TeamRoleRepository: TeamRoleRepositoryTrait> UpdateTeamRoleService<TeamRoleRepository> {
    pub fn new(team_role_repository: TeamRoleRepository) -> Self {
        UpdateTeamRoleService {
            team_role_repository,
        }
    }

    pub async fn exec(&self, params: UpdateTeamRoleParams) -> Result<TeamRole, SamambaiaError> {
        let user_can_update_team_role = verify_role_has_permission(
            &params.staff_role,
            crate::util::RolePermissions::UpdateTeamRole,
        );

        if !user_can_update_team_role {
            return Err(SamambaiaError::unauthorized_err());
        }

        let team_role_on_db = self
            .team_role_repository
            .find_by_id(params.team_role_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Update Article Service, while finding article by id",
                    err,
                )
            })?;

        if team_role_on_db.is_none() {
            return Err(SamambaiaError::resource_not_found_err());
        }

        let mut team_role = team_role_on_db.unwrap();

        if params.title.is_some() {
            team_role.set_title(params.title.unwrap());
        }

        if params.description.is_some() {
            team_role.set_description(params.description.unwrap());
        }

        self.team_role_repository
            .save(team_role)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Update Article Service, while finding article by id",
                    err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::domain::repositories::team_role_repository::MockTeamRoleRepositoryTrait;

    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        // INITIALIZATING
        let mut mocked_team_role_repository = MockTeamRoleRepositoryTrait::new();

        let team_role = TeamRole::new(
            "Ceo".into(),
            "Responsável por gerenciar todo o fã-site.".into(),
        );

        type TeamRoleDB = Arc<Mutex<Vec<TeamRole>>>;
        let team_role_db: TeamRoleDB = Arc::new(Mutex::new(Vec::new()));

        team_role_db.lock().unwrap().push(team_role.clone());

        // MOCKING TEAM ROLE METHODS
        let tr_db_clone = Arc::clone(&team_role_db);
        mocked_team_role_repository
            .expect_find_by_id()
            .returning(move |id| {
                for team_role in tr_db_clone.lock().unwrap().iter() {
                    if team_role.id().eq(&id) {
                        return Ok(Some(team_role.clone()));
                    }
                }

                Ok(None)
            });

        let tr_db_clone = Arc::clone(&team_role_db);
        mocked_team_role_repository
            .expect_save()
            .returning(move |team_role| {
                let mut index: Option<usize> = None;

                for (i, db_tr) in tr_db_clone.lock().unwrap().iter().enumerate() {
                    if db_tr.id().eq(&team_role.id()) {
                        index = Some(i);
                        break;
                    }
                }

                tr_db_clone.lock().unwrap()[index.unwrap()] = team_role.clone();

                Ok(team_role)
            });

        // TESTING

        let sut = UpdateTeamRoleService::new(mocked_team_role_repository);

        // should fail because admins cannot update team roles
        let result = sut
            .exec(UpdateTeamRoleParams {
                staff_role: Role::Admin,
                team_role_id: team_role.id(),
                title: Some("CEO".into()),
                description: None,
            })
            .await;

        assert!(result.is_err());

        // should be successfull
        let result = sut
            .exec(UpdateTeamRoleParams {
                staff_role: Role::Principal,
                team_role_id: team_role.id(),
                title: Some("CEO".into()),
                description: None,
            })
            .await;

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.title(), "CEO".to_string());
        assert_eq!(
            result.description(),
            "Responsável por gerenciar todo o fã-site.".to_string()
        );
        assert_eq!(result, team_role_db.lock().unwrap()[0].clone());

        let result = sut
            .exec(UpdateTeamRoleParams {
                staff_role: Role::Principal,
                team_role_id: team_role.id(),
                title: None,
                description: Some("Nova descrição.".to_string()),
            })
            .await;
        let result = result.unwrap();

        assert_eq!(result.description(), "Nova descrição.".to_string());
        assert_eq!(result, team_role_db.lock().unwrap()[0].clone());
    }
}
