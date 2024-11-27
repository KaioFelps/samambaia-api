use crate::domain::domain_entities::announcement::Announcement;
use crate::domain::repositories::announcements_repository::MockAnnouncementRepositoryTrait;
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

    (db, repository)
}
