use std::error::Error;
use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::comment_report::DraftCommentReport;
use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::domain::repositories::{comment_report_repository::CommentReportRepositoryTrait, user_repository::UserRepositoryTrait};
use crate::errors::bad_request_error::BadRequestError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};

use crate::{LOG_SEP, R_EOL};

pub struct CreateCommentReportParams {
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub content: String,
}
pub struct CreateCommentReportService<
UR: UserRepositoryTrait,
CR: CommentRepositoryTrait,
CRR: CommentReportRepositoryTrait,
> {
    user_repository: Box<UR>,
    comment_repository: Box<CR>,
    comment_report_repository: Box<CRR>
}

impl<
UR: UserRepositoryTrait,
CR: CommentRepositoryTrait,
CRR: CommentReportRepositoryTrait
>
CreateCommentReportService<UR, CR, CRR> {
    pub fn new(
        user_repository: Box<UR>,
        comment_repository: Box<CR>,
        comment_report_repository: Box<CRR>
    ) -> Self {
        CreateCommentReportService {
            user_repository,
            comment_repository,
            comment_report_repository
        }
    }

    pub async fn exec(&self, params: CreateCommentReportParams) -> Result<CommentReport, Box<dyn Error>> {
        let user_on_db = self.user_repository.find_by_id(&params.user_id).await;

        if user_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Comment Report Service, while fetching user from database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                user_on_db.as_ref().unwrap_err()
            );

            return Err(
                Box::new(InternalError::new())
            );
        }

        let user_on_db = user_on_db.unwrap().to_owned();

        if user_on_db.is_none() {
            return Err(
                Box::new(UnauthorizedError::new())
            )
        }

        let user_on_db = user_on_db.unwrap();
        let user_id = user_on_db.id();

        let comment_on_db = self.comment_repository.find_by_id(params.comment_id).await;
        
        if comment_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Comment Report Service, while fetching comment from database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                comment_on_db.as_ref().unwrap_err()
            );

            return Err(
                Box::new(InternalError::new())
            );
        }

        let comment_on_db = comment_on_db.unwrap();

        if comment_on_db.is_none() {
            return Err(
                Box::new(BadRequestError::new())
            )
        }

        let comment_on_db = comment_on_db.unwrap();
        let comment_id = comment_on_db.id();

        let comment_report = DraftCommentReport::new(
            comment_id,
            user_id,
            params.content,
        );

        let response = self.comment_report_repository.create(comment_report).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Comment Report Service, while creating the comment report:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );
            return Err(Box::new(InternalError::new()));
        }
        
        return Ok(response.unwrap());
    }
}


#[cfg(test)]
mod test {
    use std::sync::Mutex;
    use std::sync::Arc;

    use chrono::Utc;
    use uuid::Uuid;

    use crate::domain::domain_entities::comment::Comment;
    use crate::domain::domain_entities::comment_report::CommentReportTrait;
    use crate::domain::domain_entities::comment_report::DraftCommentReport;
    use crate::domain::repositories::{comment_repository::MockCommentRepositoryTrait, user_repository::MockUserRepositoryTrait};
    use crate::domain::repositories::comment_report_repository::MockCommentReportRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;
    use super::{CommentReport, CreateCommentReportParams};

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();
        let mut mocked_comment_repo: MockCommentRepositoryTrait = MockCommentRepositoryTrait::new();
        let mut mocked_comment_report_repo: MockCommentReportRepositoryTrait = MockCommentReportRepositoryTrait::new();

        type DB = Arc<Mutex<Vec<CommentReport>>>;

        let db: DB = Arc::new(Mutex::new(vec![]));

        mocked_user_repo
        .expect_find_by_id()
        .returning(|id| {
            let fake_user = User::new_from_existing(
                id.clone().to_owned(),
                "Fake name".to_string(),
                "password".to_string(),
                chrono::Utc::now().naive_utc(),
                None,
                Some(Role::Writter)
            );
            
            Ok(Some(fake_user))
        });

        mocked_comment_repo
        .expect_find_by_id()
        .returning(|id| {
            let fake_comm = Comment::new_from_existing(
                id,
                Uuid::new_v4(),
                Uuid::new_v4(),
                "notíca de um autor de merda fodido".into(),
                true,
                Utc::now().naive_utc()
            );

            Ok(Some(fake_comm))
        });

        let db_clone = Arc::clone(&db);
        mocked_comment_report_repo
        .expect_create()
        .returning(move |comment_report: DraftCommentReport| {
            let comment = comment_report.to_comment_report(1);
            db_clone.lock().unwrap().push(comment.clone());

            Ok(comment)
        })
        .times(1);

        let service = super::CreateCommentReportService {
            user_repository: Box::new(mocked_user_repo),
            comment_repository: Box::new(mocked_comment_repo),
            comment_report_repository: Box::new(mocked_comment_report_repo)
        };

        let result = service.exec(CreateCommentReportParams {
            comment_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            content: "Esse comentário é tóxico e ofensivo.".into()
        }).await;

        let result = result.unwrap();

        assert_eq!("Esse comentário é tóxico e ofensivo.", result.message());

        assert_eq!(result, db.lock().unwrap()[0]);
    }
}
