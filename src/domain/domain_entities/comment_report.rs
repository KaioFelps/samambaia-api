use chrono::{NaiveDateTime as DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DraftCommentReport {
    comment_id: Uuid,
    user_id: Uuid,
    message: String,
    solved: bool,
    created_at: DateTime
}

impl DraftCommentReport {
    // CONSTRUCTOR
    pub fn new(
        comment_id: Uuid,
        user_id: Uuid,
        message: String,
    ) -> Self {
        let solved = false;
        let created_at  = Utc::now().naive_utc();

        DraftCommentReport {
            comment_id,
            user_id,
            message,
            solved,
            created_at
        }
    }

    // METHODS

    pub fn to_comment_report(self, id: i32) -> CommentReport {
        CommentReport {
            id,
            user_id: self.user_id,
            comment_id: self.comment_id,
            message: self.message,
            solved: self.solved,
            created_at: self.created_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommentReport {
    id: i32,
    comment_id: Uuid,
    user_id: Uuid,
    message: String,
    solved: bool,
    created_at: DateTime
}

impl CommentReportIdTrait for CommentReport {
    fn id(&self) -> i32 {
        self.id
    }
}

impl CommentReport {
    pub fn new_from_existing(
        id: i32,
        comment_id: Uuid,
        user_id: Uuid,
        message: String,
        solved: bool,
        created_at: DateTime
    ) -> Self {
        CommentReport {
            id,
            user_id,
            comment_id,
            message,
            solved,
            created_at,
        }
    }

    // SETTERS

    pub fn set_solved(&mut self, value: bool) -> () {
        self.solved = value;
    }
}

#[duplicate::duplicate_item(comment_report; [DraftCommentReport]; [CommentReport])]
impl CommentReportTrait for comment_report {
    fn comment_id(&self) -> Uuid { self.comment_id }    

    fn user_id(&self) -> Uuid { self.user_id }

    fn message(&self) -> String { self.message.clone() }

    fn solved(&self) -> bool { self.solved }

    fn created_at(&self) -> DateTime { self.created_at }
}

pub trait CommentReportTrait {
    fn comment_id(&self) -> Uuid;
    fn user_id(&self) -> Uuid;
    fn message(&self) -> String;
    fn solved(&self) -> bool;
    fn created_at(&self) -> DateTime;
}

pub trait CommentReportIdTrait {
    fn id(&self) -> i32;
}