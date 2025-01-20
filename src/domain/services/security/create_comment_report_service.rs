use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::domain_entities::comment_report::DraftCommentReport;
use crate::domain::repositories::comment_report_repository::CommentReportRepositoryTrait;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::error::SamambaiaError;

use crate::{LOG_SEP, R_EOL};

pub struct CreateCommentReportParams {
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub content: String,
}
pub struct CreateCommentReportService<CR: CommentRepositoryTrait, CRR: CommentReportRepositoryTrait>
{
    comment_repository: CR,
    comment_report_repository: CRR,
}

impl<CR: CommentRepositoryTrait, CRR: CommentReportRepositoryTrait>
    CreateCommentReportService<CR, CRR>
{
    pub fn new(comment_repository: CR, comment_report_repository: CRR) -> Self {
        CreateCommentReportService {
            comment_repository,
            comment_report_repository,
        }
    }

    pub async fn exec(
        &self,
        params: CreateCommentReportParams,
    ) -> Result<CommentReport, SamambaiaError> {
        let comment_on_db = self.comment_repository.find_by_id(params.comment_id).await;

        if comment_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Comment Report Service, while fetching comment from database:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                comment_on_db.as_ref().unwrap_err()
            );

            return Err(SamambaiaError::internal_err());
        }

        let comment_on_db = comment_on_db.unwrap();

        if comment_on_db.is_none() {
            return Err(SamambaiaError::bad_request_err());
        }

        let comment_on_db = comment_on_db.unwrap();
        let comment_id = comment_on_db.id();

        let comment_report = DraftCommentReport::new(comment_id, params.user_id, params.content);

        let response = self.comment_report_repository.create(comment_report).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Comment Report Service, while creating the comment report:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );
            return Err(SamambaiaError::internal_err());
        }

        Ok(response.unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::sync::Mutex;

    use uuid::Uuid;

    use super::{CommentReport, CreateCommentReportParams};
    use crate::domain::domain_entities::comment::Comment;
    use crate::domain::domain_entities::comment_report::CommentReportTrait;
    use crate::domain::domain_entities::comment_report::DraftCommentReport;
    use crate::domain::repositories::comment_report_repository::MockCommentReportRepositoryTrait;
    use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;
    use crate::libs::time::TimeHelper;

    #[tokio::test]
    async fn test() {
        let mut mocked_comment_repo: MockCommentRepositoryTrait = MockCommentRepositoryTrait::new();
        let mut mocked_comment_report_repo: MockCommentReportRepositoryTrait =
            MockCommentReportRepositoryTrait::new();

        type DB = Arc<Mutex<Vec<CommentReport>>>;

        let db: DB = Arc::new(Mutex::new(vec![]));

        mocked_comment_repo.expect_find_by_id().returning(|id| {
            let fake_comm = Comment::new_from_existing(
                id,
                Some(Uuid::new_v4()),
                Uuid::new_v4(),
                "notíca de um autor de merda fodido".into(),
                true,
                TimeHelper::now(),
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
            comment_repository: mocked_comment_repo,
            comment_report_repository: mocked_comment_report_repo,
        };

        let result = service
            .exec(CreateCommentReportParams {
                comment_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                content: "Esse comentário é tóxico e ofensivo.".into(),
            })
            .await;

        let result = result.unwrap();

        assert_eq!("Esse comentário é tóxico e ofensivo.", result.message());

        assert_eq!(result, db.lock().unwrap()[0]);
    }
}
