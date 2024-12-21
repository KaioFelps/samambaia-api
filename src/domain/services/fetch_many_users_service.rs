use crate::core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE};
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::{
    FindManyUsersResponse, UserQueryType, UserRepositoryTrait,
};
use crate::error::DomainError;
use crate::util::generate_service_internal_error;

#[derive(Debug)]
pub struct FetchManyUsersResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<User>,
}

pub struct FetchManyUsersParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<UserQueryType>,
}

pub struct FetchManyUsersService<UserRepository: UserRepositoryTrait> {
    user_repository: UserRepository,
}

impl<UserRepository: UserRepositoryTrait> FetchManyUsersService<UserRepository> {
    pub fn new(user_repository: UserRepository) -> Self {
        FetchManyUsersService { user_repository }
    }

    pub async fn exec(
        &self,
        params: FetchManyUsersParams,
    ) -> Result<FetchManyUsersResponse, DomainError> {
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
            .user_repository
            .find_many(PaginationParameters {
                items_per_page,
                page,
                query: params.query,
            })
            .await;

        if let Err(err) = response {
            return Err(generate_service_internal_error(
                "Error occurred on Fetch Many Users Service, while selecting many users from the database",
                err,
            ));
        }

        let response = response.unwrap();
        let FindManyUsersResponse(users, total_items) = response;

        Ok(FetchManyUsersResponse {
            data: users,
            pagination: PaginationResponse::new(page, total_items, items_per_page),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;

    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio;

    #[tokio::test]
    async fn test() {
        // MOCKING THE DATABASE SEED ROLES
        let user_1 = User::new("Flori".into(), "vsjkvdsknjsd".into(), Some(Role::Admin));
        let user_2 = User::new("Parme".into(), "vsjkvdsknjsd".into(), Some(Role::Coord));

        // MOCKING THE DATABASE
        let user_db: Arc<Mutex<Vec<User>>> =
            Arc::new(Mutex::new(vec![user_1.clone(), user_2.clone()]));

        // MOCKING THE REPOSITORY
        let mut mocked_user_repository = MockUserRepositoryTrait::new();

        let to_move_tr_db = Arc::clone(&user_db);
        mocked_user_repository
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
                        UserQueryType::Nickname(nickname) => {
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
                        UserQueryType::Role(role) => {
                            for tr in to_move_tr_db.lock().unwrap().iter() {
                                if tr.role().unwrap().eq(&role) {
                                    users.push(tr.clone());
                                }
                            }
                        }
                    }
                }

                let total_before_paginating = users.len();

                let leap = (page - 1) * items_per_page;

                let mut res_users = vec![];

                for (index, item) in users.iter().enumerate() {
                    if index >= leap as usize {
                        res_users.push(item.to_owned());
                    }
                }

                Ok(FindManyUsersResponse(
                    res_users,
                    total_before_paginating as u64,
                ))
            });

        // TESTING

        let sut = FetchManyUsersService::new(mocked_user_repository);

        let result_1 = sut
            .exec(FetchManyUsersParams {
                page: None,
                per_page: None,
                query: Some(UserQueryType::Nickname("parme".to_string())),
            })
            .await;

        assert!(result_1.is_ok());

        let result_1 = result_1.unwrap();

        assert_eq!(result_1.pagination.total_items, 1);
        assert_eq!(result_1.data[0].nickname(), "Parme".to_string());

        let result_2 = sut
            .exec(FetchManyUsersParams {
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
