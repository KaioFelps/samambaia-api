use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::comment_report_repository::CommentReportRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct DeleteCommentReportParams {
    pub staff_role: Role,
    pub com_report_id: i32,
}

pub struct DeleteCommentReportService<CommentReportRepository: CommentReportRepositoryTrait> {
    comment_report_repository: CommentReportRepository,
}

impl<CommentReportRepository: CommentReportRepositoryTrait>
    DeleteCommentReportService<CommentReportRepository>
{
    pub fn new(comment_report_repository: CommentReportRepository) -> Self {
        DeleteCommentReportService {
            comment_report_repository,
        }
    }

    pub async fn exec(&self, params: DeleteCommentReportParams) -> Result<(), SamambaiaError> {
        let staff_can_delete =
            verify_role_has_permission(&params.staff_role, RolePermissions::DeleteReport);

        if !staff_can_delete {
            return Err(SamambaiaError::unauthorized_err());
        }

        let comm_report = self
            .comment_report_repository
            .find_by_id(params.com_report_id)
            .await.map_err(|err| generate_service_internal_error(
                "Error occurred on Delete Comment Report Service, while fetching the comment report from database",
                err
            ))?;

        if comm_report.is_none() {
            return Err(SamambaiaError::bad_request_err());
        }

        let comm_report = comm_report.unwrap();

        self
            .comment_report_repository
            .delete(comm_report).await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Delete Comment Report Service, while updating the comment report at database",
                err,
            ))
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use tokio;
    use uuid::Uuid;

    use super::*;
    use crate::domain::domain_entities::comment_report::{CommentReport, CommentReportIdTrait};
    use crate::domain::repositories::comment_report_repository::MockCommentReportRepositoryTrait;
    use crate::libs::time::TimeHelper;

    #[tokio::test]
    async fn test() {
        let mut mocked_comment_report_repo = MockCommentReportRepositoryTrait::new();

        type CommentReportDB = Arc<Mutex<Vec<CommentReport>>>;

        let comm_report_db: CommentReportDB = Arc::new(Mutex::new(vec![]));

        let comment_report_1 = CommentReport::new_from_existing(
            1,
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Esse comentário é tóxico.".into(),
            None,
            TimeHelper::now(),
        );

        let comment_report_id_1 = comment_report_1.id();

        let comment_report_2 = CommentReport::new_from_existing(
            2,
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Estão me ofendendo neste comentário!".into(),
            None,
            TimeHelper::now(),
        );

        let comment_report_id_2 = comment_report_2.id();

        comm_report_db.lock().unwrap().push(comment_report_1);
        comm_report_db.lock().unwrap().push(comment_report_2);

        let comm_report_db_clone = Arc::clone(&comm_report_db);
        mocked_comment_report_repo
            .expect_find_by_id()
            .returning(move |id| {
                let mut _comm_report: Option<CommentReport> = None;

                for comm_rep in comm_report_db_clone.lock().unwrap().iter() {
                    if comm_rep.id().eq(&id) {
                        _comm_report = Some(comm_rep.clone());
                        break;
                    }
                }

                Ok(_comm_report)
            });

        let comm_report_db_clone = Arc::clone(&comm_report_db);
        mocked_comment_report_repo
            .expect_delete()
            .returning(move |comm_report| {
                let mut new_db = vec![];

                for comm_rep in comm_report_db_clone.lock().unwrap().iter() {
                    if !comm_rep.id().eq(&comm_report.id()) {
                        new_db.push(comm_rep.clone())
                    }
                }

                *comm_report_db_clone.lock().unwrap() = new_db;

                Ok(())
            });

        let sut = DeleteCommentReportService {
            comment_report_repository: mocked_comment_report_repo,
        };

        let result = sut
            .exec(DeleteCommentReportParams {
                staff_role: Role::Principal,
                com_report_id: comment_report_id_1,
            })
            .await;

        assert!(result.is_ok());
        assert_ne!(comment_report_id_1, comm_report_db.lock().unwrap()[0].id());

        let result_2 = sut
            .exec(DeleteCommentReportParams {
                staff_role: Role::User,
                com_report_id: comment_report_id_2,
            })
            .await;

        assert!(result_2.is_err());
        assert_eq!(comment_report_id_2, comm_report_db.lock().unwrap()[0].id());
    }
}
