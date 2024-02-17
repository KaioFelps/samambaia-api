use chrono::{NaiveDateTime as DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamUser {
    id: Uuid,
    nickname: String,
    task: String,
    twitter: Option<String>,
    discord: Option<String>,
    created_at: DateTime
}

impl TeamUser {
    // CONSTRUCTORS
    pub fn new(
        nickname: String,
        task: String,
        twitter: Option<String>,
        discord: Option<String>,
    ) -> Self {
        let id = Uuid::new_v4();
        let created_at  = Utc::now().naive_utc();

        TeamUser {
            id,
            nickname,
            task,
            twitter,
            discord,
            created_at,
        }
    }

    pub fn new_from_existing(
        id: Uuid,
        nickname: String,
        task: String,
        twitter: Option<String>,
        discord: Option<String>,
        created_at: DateTime
    ) -> Self {
        TeamUser {
            id,
            nickname,
            task,
            twitter,
            discord,
            created_at,
        }
    }

    // GETTERS
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn nickname(&self) -> &str {
        self.nickname.as_ref()
    }

    pub fn task(&self) -> &str {
        self.task.as_ref()
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

    pub fn set_task(&mut self, task: String) {
        self.task = task;
    }

    pub fn set_twitter(&mut self, twitter: Option<String>) {
        self.twitter = twitter;
    }

    pub fn set_discord(&mut self, discord: Option<String>) {
        self.discord = discord;
    }

}