use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::domain_entities::council_alert::CouncilAlert;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize, Deserialize)]
pub struct MappedCouncilAlert {
    id: i32,
    pinned: bool,
    title: String,
    content: String,
    #[serde(rename = "createdAt")]
    created_at: NaiveDateTime,
}

pub struct CouncilAlertPresenter;

impl PresenterTrait<CouncilAlert, MappedCouncilAlert> for CouncilAlertPresenter {
    fn to_http(council_alert: CouncilAlert) -> MappedCouncilAlert {
        MappedCouncilAlert {
            id: council_alert.id(),
            pinned: council_alert.pinned(),
            title: council_alert.title().to_string(),
            content: council_alert.content().to_string(),
            created_at: council_alert.created_at(),
        }
    }
}
