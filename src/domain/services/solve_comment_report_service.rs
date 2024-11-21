use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::comment_report_repository::CommentReportRepositoryTrait;
use crate::error::DomainError;
use crate::util::generate_service_internal_error;
use crate::util::verify_role_has_permission;
use crate::util::RolePermissions;

pub struct SolveCommentReportParams {
    pub staff_id: Uuid,
    pub staff_role: Role,
    pub com_report_id: i32,
}

pub struct SolveCommentReportService<CommentReportRepository: CommentReportRepositoryTrait> {
    comment_report_repository: Box<CommentReportRepository>,
}

impl<CommentReportRepository: CommentReportRepositoryTrait>
    SolveCommentReportService<CommentReportRepository>
{
    pub fn new(comment_report_repository: Box<CommentReportRepository>) -> Self {
        SolveCommentReportService {
            comment_report_repository,
        }
    }

    pub async fn exec(&self, params: SolveCommentReportParams) -> Result<(), DomainError> {
        let staff_can_solve =
            verify_role_has_permission(&params.staff_role, RolePermissions::SolveReport);

        if !staff_can_solve {
            return Err(DomainError::unauthorized_err());
        }

        let comm_report = self
            .comment_report_repository
            .find_by_id(params.com_report_id)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Solve Comment Report Service, while fetching the comment report from database",
                err,
            ))?;

        if comm_report.is_none() {
            return Err(DomainError::resource_not_found_err());
        }

        let mut comm_report = comm_report.unwrap();

        comm_report.set_solved_by(Some(params.staff_id));

        let _comment_report = self.comment_report_repository.save(comm_report).await.map_err(|err| generate_service_internal_error(
            "Error occurred on Solve Comment Report Service, while updating the comment report at database",
            err,
        ))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, Mutex};

    use crate::domain::domain_entities::comment_report::{
        CommentReport, CommentReportIdTrait, CommentReportTrait,
    };
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::comment_report_repository::MockCommentReportRepositoryTrait;
    use crate::libs::time::TimeHelper;

    use tokio;

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
            .expect_save()
            .returning(move |comm_report| {
                let mut index = None;

                for (i, comm_rep) in comm_report_db_clone.lock().unwrap().iter().enumerate() {
                    if comm_rep.id().eq(&comm_report.id()) {
                        index = Some(i);
                    }
                }

                if index.is_some() {
                    comm_report_db_clone.lock().unwrap()[index.unwrap()] = comm_report.clone();
                }

                Ok(comm_report)
            });

        let sut = SolveCommentReportService {
            comment_report_repository: Box::new(mocked_comment_report_repo),
        };

        let fake_staff_id = Uuid::new_v4();

        let result = sut
            .exec(SolveCommentReportParams {
                staff_id: fake_staff_id,
                staff_role: Role::Coord,
                com_report_id: comment_report_id_1,
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(
            Some(fake_staff_id),
            comm_report_db.lock().unwrap()[0].solved_by()
        );

        let result_2 = sut
            .exec(SolveCommentReportParams {
                staff_id: fake_staff_id,
                staff_role: Role::User,
                com_report_id: comment_report_id_2,
            })
            .await;

        assert!(result_2.is_err());
        assert_eq!(None, comm_report_db.lock().unwrap()[1].solved_by());
    }
}
