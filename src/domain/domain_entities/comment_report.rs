use chrono::NaiveDateTime as DateTime;
use uuid::Uuid;

use crate::libs::time::TimeHelper;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DraftCommentReport {
    comment_id: Uuid,
    user_id: Uuid,
    message: String,
    solved_by: Option<Uuid>,
    created_at: DateTime,
}

impl DraftCommentReport {
    // CONSTRUCTOR
    pub fn new(comment_id: Uuid, user_id: Uuid, message: String) -> Self {
        let solved_by = None;
        let created_at = TimeHelper::now();

        DraftCommentReport {
            comment_id,
            user_id,
            message,
            solved_by,
            created_at,
        }
    }

    // METHODS
    pub fn to_comment_report(self, id: i32) -> CommentReport {
        CommentReport {
            id,
            user_id: self.user_id,
            comment_id: self.comment_id,
            message: self.message,
            solved_by: self.solved_by,
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
    solved_by: Option<Uuid>,
    created_at: DateTime,
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
        solved_by: Option<Uuid>,
        created_at: DateTime,
    ) -> Self {
        CommentReport {
            id,
            user_id,
            comment_id,
            message,
            solved_by,
            created_at,
        }
    }

    // SETTERS

    pub fn set_solved_by(&mut self, value: Option<Uuid>) {
        self.solved_by = value;
    }
}

#[duplicate::duplicate_item(comment_report; [DraftCommentReport]; [CommentReport])]
impl CommentReportTrait for comment_report {
    fn comment_id(&self) -> Uuid {
        self.comment_id
    }

    fn user_id(&self) -> Uuid {
        self.user_id
    }

    fn message(&self) -> String {
        self.message.clone()
    }

    fn solved_by(&self) -> Option<Uuid> {
        self.solved_by
    }

    fn created_at(&self) -> DateTime {
        self.created_at
    }
}

pub trait CommentReportTrait {
    fn comment_id(&self) -> Uuid;
    fn user_id(&self) -> Uuid;
    fn message(&self) -> String;
    fn solved_by(&self) -> Option<Uuid>;
    fn created_at(&self) -> DateTime;
}

pub trait CommentReportIdTrait {
    fn id(&self) -> i32;
}
