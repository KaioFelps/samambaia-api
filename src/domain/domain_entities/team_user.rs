use crate::libs::time::TimeHelper;
use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamUser {
    id: Uuid,
    team_role_id: Uuid,
    nickname: String,
    user_function: String,
    twitter: Option<String>,
    discord: Option<String>,
    created_at: DateTime,
}

impl TeamUser {
    // CONSTRUCTORS
    pub fn new(
        nickname: String,
        user_function: String,
        twitter: Option<String>,
        discord: Option<String>,
        team_role_id: Uuid,
    ) -> Self {
        let id = Uuid::new_v4();
        let created_at = TimeHelper::now();

        TeamUser {
            id,
            team_role_id,
            nickname,
            user_function,
            twitter,
            discord,
            created_at,
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        team_role_id: Uuid,
        nickname: String,
        user_function: String,
        twitter: Option<String>,
        discord: Option<String>,
        created_at: DateTime,
    ) -> Self {
        TeamUser {
            id,
            team_role_id,
            nickname,
            user_function,
            twitter,
            discord,
            created_at,
        }
    }

    // GETTERS
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn team_role_id(&self) -> Uuid {
        self.team_role_id
    }

    pub fn nickname(&self) -> &str {
        self.nickname.as_ref()
    }

    pub fn user_function(&self) -> &str {
        self.user_function.as_ref()
    }

    pub fn twitter(&self) -> Option<String> {
        self.twitter.clone()
    }

    pub fn discord(&self) -> Option<String> {
        self.discord.clone()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at
    }

    // SETTERS
    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }

    pub fn set_user_function(&mut self, user_function: String) {
        self.user_function = user_function;
    }

    pub fn set_twitter(&mut self, twitter: Option<String>) {
        self.twitter = twitter;
    }

    pub fn set_discord(&mut self, discord: Option<String>) {
        self.discord = discord;
    }

    pub fn set_team_role_id(&mut self, team_role_id: Uuid) {
        self.team_role_id = team_role_id;
    }
}
