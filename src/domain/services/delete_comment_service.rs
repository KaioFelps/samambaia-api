use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::error::DomainError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct DeleteCommentParams {
    pub staff_role: Role,
    pub user_id: Uuid,
    pub comment_id: Uuid,
}
pub struct DeleteCommentService<CommentRepository: CommentRepositoryTrait> {
    comment_repository: Box<CommentRepository>,
}

impl<CommentRepository: CommentRepositoryTrait> DeleteCommentService<CommentRepository> {
    pub fn new(comment_repository: Box<CommentRepository>) -> Self {
        DeleteCommentService { comment_repository }
    }

    pub async fn exec(&self, params: DeleteCommentParams) -> Result<(), DomainError> {
        let comment_on_db = self
            .comment_repository
            .find_by_id(params.comment_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Delete Comment Service, while finding comment by Id",
                    err,
                )
            })?;

        if comment_on_db.is_none() {
            return Err(DomainError::resource_not_found_err());
        }

        let comment = comment_on_db.unwrap();

        // checks user is allowed to perform the update
        let user_can_delete =
            verify_role_has_permission(&params.staff_role, RolePermissions::DeleteComment);

        if !user_can_delete && comment.author_id() != params.user_id {
            return Err(DomainError::unauthorized_err());
        }

        self.comment_repository
            .delete(comment)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Delete Comment Service, while deleting the comment",
                    err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use tokio;
    use uuid::Uuid;

    use super::{DeleteCommentParams, DeleteCommentService};

    use crate::domain::domain_entities::comment::Comment;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;

    #[tokio::test]
    async fn test() {
        let mut mocked_comment_repo: MockCommentRepositoryTrait = MockCommentRepositoryTrait::new();

        let comment = Comment::new(
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            "Conteúdo inicial".to_string(),
        );

        let comment_db: Arc<Mutex<Vec<Comment>>> = Arc::new(Mutex::new(vec![comment.clone()]));

        // mocking comment repo
        let mocked_comment_repo_db_clone = Arc::clone(&comment_db);
        mocked_comment_repo
            .expect_find_by_id()
            .returning(move |id| {
                let comment_db = mocked_comment_repo_db_clone.lock().unwrap();

                for item in comment_db.iter() {
                    if item.id() == id {
                        return Ok(Some(item.clone()));
                    }
                }

                Ok(None)
            });

        let mocked_comment_repo_db_clone = Arc::clone(&comment_db);
        mocked_comment_repo
            .expect_delete()
            .returning(move |_comment| {
                let mut comment_db = mocked_comment_repo_db_clone.lock().unwrap();
                comment_db.truncate(0);

                Ok(())
            });

        let service = DeleteCommentService {
            comment_repository: Box::new(mocked_comment_repo),
        };

        let result = service.exec(DeleteCommentParams {
            user_id: comment.author_id(),
            staff_role: Role::User,
            comment_id: comment.id(),
        });

        tokio::try_join!(result).unwrap();

        let db = comment_db.lock().unwrap();
        assert_eq!(0, db.len());
    }
}
