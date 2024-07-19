use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime as DateTime;

use crate::domain::domain_entities::team_user::TeamUser;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize, Deserialize)]
pub struct MappedTeamUser {
    nickname: String,
    #[serde(rename = "function")]
    user_function: String,
    twitter: Option<String>,
    discord: Option<String>,
    #[serde(rename = "hiredAt")]
    created_at: DateTime
}

pub struct TeamUserPresenter;

impl PresenterTrait<TeamUser, MappedTeamUser> for TeamUserPresenter {
    fn to_http(user: TeamUser) -> MappedTeamUser {
        MappedTeamUser {
            discord: user.discord(),
            nickname: user.nickname().to_owned(),
            created_at: user.created_at(),
            user_function: user.user_function().to_owned(),
            twitter: user.twitter(),
        }
    }
}
