use std::sync::{Arc, RwLock};

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment::Comment;
use crate::domain::domain_entities::team_user::TeamUser;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::summary_repository::MockSummaryRepositoryTrait;
use crate::domain::value_objects::count_summary::CountSummary;

pub struct SummaryDbTables {
    pub user: Vec<User>,
    pub team_user: Vec<TeamUser>,
    pub article: Vec<Article>,
    pub comment: Vec<Comment>,
}

pub fn get_summary_repository() -> (Arc<RwLock<SummaryDbTables>>, MockSummaryRepositoryTrait) {
    let db = Arc::new(RwLock::new(SummaryDbTables {
        user: Vec::new(),
        team_user: Vec::new(),
        article: Vec::new(),
        comment: Vec::new(),
    }));

    let mut repository = MockSummaryRepositoryTrait::new();

    let db_clone = Arc::clone(&db);
    repository.expect_get_table_summary().returning(move || {
        let db = db_clone.read().unwrap();
        Ok(CountSummary {
            articles: db.article.len() as i32,
            comments: db.comment.len() as i32,
            team_users: db.team_user.len() as i32,
            users: db.user.len() as i32,
        })
    });

    (db, repository)
}
