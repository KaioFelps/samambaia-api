use crate::core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE};
use crate::domain::domain_entities::team_user::TeamUser;
use crate::domain::repositories::team_user_repository::{
    FindManyTeamUsersResponse, TeamUserQueryType, TeamUserRepositoryTrait,
};
use crate::error::DomainError;
use crate::util::generate_service_internal_error;

#[derive(Debug)]
pub struct FetchManyTeamUsersResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<TeamUser>,
}

pub struct FetchManyTeamUsersParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<TeamUserQueryType>,
}

pub struct FetchManyTeamUsersService<TeamUserRepository: TeamUserRepositoryTrait> {
    team_user_repository: TeamUserRepository,
}

impl<TeamUserRepository: TeamUserRepositoryTrait> FetchManyTeamUsersService<TeamUserRepository> {
    pub fn new(team_user_repository: TeamUserRepository) -> Self {
        FetchManyTeamUsersService {
            team_user_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchManyTeamUsersParams,
    ) -> Result<FetchManyTeamUsersResponse, DomainError> {
        let items_per_page = if params.per_page.is_some() {
            params.per_page.unwrap()
        } else {
            DEFAULT_PER_PAGE as u32
        };

        let page = if params.page.is_some() {
            let params_page = params.page.unwrap();
            if params_page == 0 {
                1
            } else {
                params_page
            }
        } else {
            1
        };

        let FindManyTeamUsersResponse(team_users, total_items) = self
            .team_user_repository
            .find_many(PaginationParameters {
                items_per_page,
                page,
                query: params.query,
            })
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Fetch Many Team Users Service, while selecting many team users from the database",
                err,
            ))?;

        Ok(FetchManyTeamUsersResponse {
            data: team_users,
            pagination: PaginationResponse::new(page, total_items, items_per_page),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::team_role::TeamRole;
    use crate::domain::repositories::team_user_repository::MockTeamUserRepositoryTrait;

    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio;

    #[tokio::test]
    async fn test() {
        // MOCKING THE DATABASE SEED ROLES
        let team_role = TeamRole::new("shit".into(), "hahahahahahaha".into());

        let team_user_1 = TeamUser::new(
            "Flori".into(),
            "vsjkvdsknjsd".into(),
            None,
            None,
            team_role.id(),
        );
        let team_user_2 = TeamUser::new(
            "Parme".into(),
            "vsjkvdsknjsd".into(),
            None,
            None,
            team_role.id(),
        );

        // MOCKING THE DATABASE
        let team_user_db: Arc<Mutex<Vec<TeamUser>>> =
            Arc::new(Mutex::new(vec![team_user_1.clone(), team_user_2.clone()]));

        // MOCKING THE REPOSITORY
        let mut mocked_team_user_repository = MockTeamUserRepositoryTrait::new();

        let to_move_tr_db = Arc::clone(&team_user_db);
        mocked_team_user_repository
            .expect_find_many()
            .returning(move |params| {
                let PaginationParameters {
                    page,
                    items_per_page,
                    query,
                } = params;

                let mut users = Vec::new();

                if query.is_none() {
                    users = to_move_tr_db.lock().unwrap().clone();
                } else {
                    match query.unwrap() {
                        TeamUserQueryType::Nickname(nickname) => {
                            for tr in to_move_tr_db.lock().unwrap().iter() {
                                if tr
                                    .nickname()
                                    .to_lowercase()
                                    .contains(&nickname.to_lowercase())
                                {
                                    users.push(tr.clone());
                                }
                            }
                        }
                        TeamUserQueryType::TeamRole(id) => {
                            for tr in to_move_tr_db.lock().unwrap().iter() {
                                if tr.team_role_id().eq(&id) {
                                    users.push(tr.clone());
                                }
                            }
                        }
                    }
                }

                let total_before_paginating = users.len();

                let leap = (page - 1) * items_per_page;

                let mut res_team_users = vec![];

                for (index, item) in users.iter().enumerate() {
                    if index >= leap as usize {
                        res_team_users.push(item.to_owned());
                    }
                }

                Ok(FindManyTeamUsersResponse(
                    res_team_users,
                    total_before_paginating as u64,
                ))
            });

        // TESTING

        let sut = FetchManyTeamUsersService::new(mocked_team_user_repository);

        let result_1 = sut
            .exec(FetchManyTeamUsersParams {
                page: None,
                per_page: None,
                query: Some(TeamUserQueryType::Nickname("parme".to_string())),
            })
            .await;

        assert!(result_1.is_ok());

        let result_1 = result_1.unwrap();

        assert_eq!(result_1.pagination.total_items, 1);
        assert_eq!(result_1.data[0].nickname(), "Parme".to_string());

        let result_2 = sut
            .exec(FetchManyTeamUsersParams {
                page: None,
                per_page: None,
                query: None,
            })
            .await;

        assert!(result_2.is_ok());

        let result_2 = result_2.unwrap();

        assert_eq!(result_2.pagination.total_items, 2);
        assert_eq!(result_2.data[0].nickname(), "Flori".to_string());
        assert_eq!(result_2.data[1].nickname(), "Parme".to_string());
    }
}
