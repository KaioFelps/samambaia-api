use std::sync::{Arc, Mutex};

pub mod announcements_repository;
pub mod article_comment_repository;
pub mod article_repository;
pub mod article_tag_repository;
pub mod comment_repository;
pub mod comment_with_user_repository;
pub mod council_alerts_repository;
pub mod free_badge_repository;
pub mod summary_repository;
pub mod users_repository;

type LocalDb<T> = Arc<Mutex<Vec<T>>>;

fn get_local_db<T>() -> LocalDb<T> {
    Arc::new(Mutex::new(Vec::new()))
}
