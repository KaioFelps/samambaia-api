use std::sync::{Arc, Mutex};

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::{
    FindManyUsersResponse,
    MockUserRepositoryTrait,
    UserQueryType,
};

pub fn get_user_repository() -> (Arc<Mutex<Vec<User>>>, MockUserRepositoryTrait) {
    let db = Arc::new(Mutex::new(vec![]));
    let mut repository = MockUserRepositoryTrait::new();

    let db_c = db.clone();
    repository.expect_create().returning(move |user: User| {
        db_c.lock().unwrap().push(user.clone());
        Ok(user)
    });

    let db_c = db.clone();
    repository.expect_find_many().returning(move |params| {
        let PaginationParameters {
            page,
            items_per_page,
            query,
        } = params;

        let db = db_c.lock().unwrap();
        let users = match query {
            None => db.clone(),
            Some(query) => match query {
                UserQueryType::Nickname(nickname) => db
                    .iter()
                    .filter(|user| {
                        user.nickname()
                            .to_lowercase()
                            .contains(&nickname.to_lowercase())
                    })
                    .cloned()
                    .collect::<Vec<User>>(),

                UserQueryType::Role(role) => db
                    .iter()
                    .filter(|user| user.role().unwrap().eq(&role))
                    .cloned()
                    .collect::<Vec<User>>(),
            },
        };

        let total_before_paginating: usize = users.len();
        let leap = (page - 1) * items_per_page;

        let res_users = users
            .iter()
            .skip(leap as usize)
            .take(items_per_page as usize)
            .cloned()
            .collect();

        Ok(FindManyUsersResponse(
            res_users,
            total_before_paginating as u64,
        ))
    });

    let db_c = db.clone();
    repository
        .expect_find_by_nickname()
        .returning(move |nickname| {
            let user = db_c
                .lock()
                .unwrap()
                .iter()
                .find(|user| user.nickname().eq(nickname))
                .cloned();

            Ok(user)
        });

    let db_c = db.clone();
    repository.expect_find_by_id().returning(move |id| {
        let user = db_c
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.id().eq(id))
            .cloned();

        Ok(user)
    });

    let db_c = db.clone();
    repository.expect_save().returning(move |p_user| {
        let mut db = db_c.lock().unwrap();
        *db = db
            .clone()
            .into_iter()
            .map(|user| {
                if user.id().eq(&p_user.id()) {
                    p_user.clone()
                } else {
                    user
                }
            })
            .collect();

        Ok(p_user)
    });

    (db, repository)
}
