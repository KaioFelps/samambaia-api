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

    (db, repository)
}
