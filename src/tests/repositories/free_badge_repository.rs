use std::sync::{Arc, Mutex};
use crate::domain::domain_entities::free_badge::FreeBadge;
use crate::domain::repositories::free_badge_repository::{FindManyFreeBadgesResponse, MockFreeBadgeRepositoryTrait};

pub fn get_free_badge_repository() -> (Arc<Mutex<Vec<FreeBadge>>>, MockFreeBadgeRepositoryTrait) {
    let db: Arc<Mutex<Vec<FreeBadge>>> = Arc::new(Mutex::new(Vec::new()));

    let mut repository = MockFreeBadgeRepositoryTrait::new();

    let db_clone = Arc::clone(&db);
    repository.expect_create().returning(move |badge| {
        db_clone.lock().unwrap().push(badge.clone());
        Ok(badge)
    });

    let db_clone = Arc::clone(&db);
    repository.expect_save().returning(move |badge| {
       let new_db = db_clone.lock().unwrap().iter().map(|item| {
           if item.id().eq(&badge.id()) {
               badge.clone()
           } else {
               item.clone()
           }
       }).collect::<Vec<FreeBadge>>();

        *db_clone.lock().unwrap() = new_db;
        Ok(badge)
    });

    let db_clone = Arc::clone(&db);
    repository.expect_delete().returning(move |badge| {
       let mut new_db = vec![];

        for item in db_clone.lock().unwrap().iter() {
           if item.ne(&badge) {
               new_db.push(item.clone());
           }
       }

        *db_clone.lock().unwrap() = new_db;
        Ok(())
    });

    let db_clone = Arc::clone(&db);
    repository.expect_find_many().returning(move |params| {
        let badges: Vec<FreeBadge> = db_clone.lock().unwrap().clone();
        let total_of_items_before_paginating = badges.len();
        let leap = (params.page - 1) * params.items_per_page;
        let mut res_badges = vec![];

        for (index, item) in badges.into_iter().enumerate() {
            if index >= leap as usize {
                res_badges.push(item);
            }
        }

        Ok(FindManyFreeBadgesResponse (res_badges, total_of_items_before_paginating as u64))
    });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_id().returning(move |free_badge_id| {
       let mut free_badge = None;

        for badge in db_clone.lock().unwrap().iter() {
            if badge.id().eq(&free_badge_id) {
                free_badge = Some(badge.clone());
                break;
            }
        }

        Ok(free_badge)
    });

    (db, repository)
}