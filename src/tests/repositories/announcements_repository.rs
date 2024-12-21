use crate::domain::repositories::announcements_repository::{
    FindManyAnnouncementsResponse, MockAnnouncementRepositoryTrait,
};
use crate::domain::{
    domain_entities::announcement::Announcement,
    repositories::announcements_repository::AnnouncementQueryType,
};
use std::sync::{Arc, Mutex};

pub fn get_announcements_repository() -> (
    Arc<Mutex<Vec<Announcement>>>,
    MockAnnouncementRepositoryTrait,
) {
    let db: Arc<Mutex<Vec<Announcement>>> = Arc::new(Mutex::new(vec![]));
    let mut repository = MockAnnouncementRepositoryTrait::new();

    let db_clone = Arc::clone(&db);
    repository
        .expect_create()
        .returning(move |announcement: Announcement| {
            db_clone.lock().unwrap().push(announcement.clone());
            Ok(announcement)
        });

    let db_clone = Arc::clone(&db);
    repository.expect_find_by_id().returning(move |id| {
        Ok(db_clone
            .lock()
            .unwrap()
            .iter()
            .find(|announcement| announcement.id().eq(id))
            .cloned())
    });

    let db_clone = Arc::clone(&db);
    repository.expect_save().returning(move |announcement| {
        let mut db = db_clone.lock().unwrap();
        *db = db
            .clone()
            .into_iter()
            .map(|db_announcement| {
                if db_announcement.id().eq(announcement.id()) {
                    announcement.clone()
                } else {
                    db_announcement
                }
            })
            .collect();

        Ok(announcement)
    });

    let db_clone = Arc::clone(&db);
    repository
        .expect_delete()
        .returning(move |announcement_id| {
            let mut db = db_clone.lock().unwrap();
            *db = db
                .clone()
                .into_iter()
                .filter(|db_announcement| db_announcement.id().ne(announcement_id))
                .collect();

            Ok(())
        });

    let db_clone = db.clone();
    repository.expect_find_many().returning(move |params| {
        let db = db_clone.lock().unwrap();

        let mut selected = match params.query {
            None => db.iter().cloned().collect::<Vec<Announcement>>(),
            Some(query) => match query {
                AnnouncementQueryType::Description(value) => db
                    .iter()
                    .filter(|announcement| {
                        announcement
                            .description()
                            .to_lowercase()
                            .contains(&value.to_lowercase())
                    })
                    .cloned()
                    .collect::<Vec<Announcement>>(),
            },
        };

        let total_items = selected.len() as u64;

        selected.sort_by(|a, b| b.created_at().cmp(a.created_at()));

        let selected = selected
            .into_iter()
            .skip(((params.page - 1) * params.items_per_page) as usize)
            .take(params.items_per_page as usize)
            .collect::<Vec<Announcement>>();

        Ok(FindManyAnnouncementsResponse(selected, total_items))
    });

    (db, repository)
}
