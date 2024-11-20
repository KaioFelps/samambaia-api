use log::error;

use crate::core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE};
use crate::domain::domain_entities::team_role::TeamRole;
use crate::domain::repositories::team_role_repository::{
    FindManyTeamRolesResponse, TeamRoleQueryType, TeamRoleRepositoryTrait,
};
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;

use crate::{LOG_SEP, R_EOL};

pub struct FetchManyTeamRolesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<TeamRoleQueryType>,
}

#[derive(Debug)]
pub struct FetchManyTeamRolesResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<TeamRole>,
}

pub struct FetchManyTeamRolesService<TeamRoleRepository: TeamRoleRepositoryTrait> {
    team_role_repository: Box<TeamRoleRepository>,
}

impl<TeamRoleRepository: TeamRoleRepositoryTrait> FetchManyTeamRolesService<TeamRoleRepository> {
    pub fn new(team_role_repository: Box<TeamRoleRepository>) -> Self {
        FetchManyTeamRolesService {
            team_role_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchManyTeamRolesParams,
    ) -> Result<FetchManyTeamRolesResponse, Box<dyn DomainErrorTrait>> {
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

        let response = self
            .team_role_repository
            .find_many(PaginationParameters {
                items_per_page,
                page,
                query: params.query,
            })
            .await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Team Roles Service, while finding many team roles from database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let response = response.unwrap();
        let FindManyTeamRolesResponse(team_roles, total_items) = response;

        Ok(FetchManyTeamRolesResponse {
            data: team_roles,
            pagination: PaginationResponse {
                current_page: page,
                total_items,
                total_pages: (total_items as f64 / items_per_page as f64).ceil() as u32,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::repositories::team_role_repository::MockTeamRoleRepositoryTrait;

    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio;

    #[tokio::test]
    async fn test() {
        // MOCKING THE DATABASE SEED ROLES
        let team_role_1 = TeamRole::new("Foo 1".into(), "Bar 1".into());
        let team_role_2 = TeamRole::new("Foo 2".into(), "Bar 2".into());

        // MOCKING THE DATABASE
        let team_role_db: Arc<Mutex<Vec<TeamRole>>> =
            Arc::new(Mutex::new(vec![team_role_1.clone(), team_role_2.clone()]));

        // MOCKING THE REPOSITORY
        let mut mocked_team_role_repository = MockTeamRoleRepositoryTrait::new();

        let to_move_tr_db = Arc::clone(&team_role_db);
        mocked_team_role_repository
            .expect_find_many()
            .returning(move |params| {
                let PaginationParameters {
                    page,
                    items_per_page,
                    query,
                } = params;

                let mut roles = Vec::new();

                if query.is_none() {
                    roles = to_move_tr_db.lock().unwrap().clone();
                } else {
                    match query.unwrap() {
                        TeamRoleQueryType::Title(title) => {
                            for tr in to_move_tr_db.lock().unwrap().iter() {
                                if tr.title().contains(&title) {
                                    roles.push(tr.clone());
                                }
                            }
                        }
                    }
                }

                let total_before_paginating = roles.len();

                let leap = (page - 1) * items_per_page;

                let mut res_team_roles = vec![];

                for (index, item) in roles.iter().enumerate() {
                    if index >= leap as usize {
                        res_team_roles.push(item.to_owned());
                    }
                }

                Ok(FindManyTeamRolesResponse(
                    res_team_roles,
                    total_before_paginating as u64,
                ))
            });

        // TESTING

        let sut = FetchManyTeamRolesService::new(Box::new(mocked_team_role_repository));

        let result_1 = sut
            .exec(FetchManyTeamRolesParams {
                page: None,
                per_page: None,
                query: Some(TeamRoleQueryType::Title("1".to_string())),
            })
            .await;

        assert!(result_1.is_ok());

        let result_1 = result_1.unwrap();

        assert_eq!(result_1.pagination.total_items, 1);
        assert_eq!(result_1.data[0].title(), "Foo 1".to_string());

        let result_2 = sut
            .exec(FetchManyTeamRolesParams {
                page: None,
                per_page: None,
                query: None,
            })
            .await;

        assert!(result_2.is_ok());

        let result_2 = result_2.unwrap();

        assert_eq!(result_2.pagination.total_items, 2);
        assert_eq!(result_2.data[0].title(), "Foo 1".to_string());
        assert_eq!(result_2.data[1].title(), "Foo 2".to_string());
    }
}
