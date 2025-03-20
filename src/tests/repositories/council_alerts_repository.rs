use std::sync::{Arc, Mutex};

use crate::domain::domain_entities::council_alert::CouncilAlert;
use crate::domain::repositories::council_alerts_repository::MockCouncilAlertRepositoryTrait;

pub fn get_council_alerts_repository() -> (
    Arc<Mutex<Vec<CouncilAlert>>>,
    MockCouncilAlertRepositoryTrait,
) {
    let db: Arc<Mutex<Vec<CouncilAlert>>> = Arc::new(Mutex::new(Vec::new()));

    let mut repository = MockCouncilAlertRepositoryTrait::new();

    let db_clone = db.clone();
    repository.expect_find_all().returning(move || {
        let (mut pinned_alerts, mut alerts): (Vec<_>, Vec<_>) = db_clone
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .partition(|alert| alert.pinned());

        alerts.sort_by_key(|alert| std::cmp::Reverse(alert.created_at()));
        pinned_alerts.sort_by_key(|alert| std::cmp::Reverse(alert.created_at()));
        pinned_alerts.append(&mut alerts);

        Ok(pinned_alerts)
    });

    let db_clone = db.clone();
    repository.expect_create().returning(move |draft| {
        let mut db = db_clone.lock().unwrap();
        let council_alert = draft.into_council_alert(db.len() as i32 + 1);
        db.push(council_alert.clone());
        Ok(council_alert)
    });

    (db, repository)
}
