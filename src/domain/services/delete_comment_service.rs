use std::error::Error;
use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::role::Role;
use crate::{LOG_SEP, R_EOL};

use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};
use crate::util::{RolePermissions, verify_role_has_permission};

pub struct DeleteCommentParams {
    pub staff_role: Role,
    pub user_id: Uuid,
    pub comment_id: Uuid,
}
pub struct DeleteCommentService<CommentRepository: CommentRepositoryTrait> {
    comment_repository: Box<CommentRepository>,
}

impl<CommentRepository: CommentRepositoryTrait>
DeleteCommentService<CommentRepository>
{
    pub fn new(comment_repository: Box<CommentRepository>) -> Self {
        DeleteCommentService {
            comment_repository,
        }
    }

    pub async fn exec(&self, params: DeleteCommentParams) -> Result<(), Box<dyn Error>> {
        let comment_on_db = self.comment_repository.find_by_id(params.comment_id).await;

        if comment_on_db.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Comment Service, while finding comment by Id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                comment_on_db.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }
        
        let comment_on_db = comment_on_db.unwrap();

        if comment_on_db.is_none() { return Err(Box::new(ResourceNotFoundError::new())) }

        let comment = comment_on_db.unwrap();

        // checks user is allowed to perform the update
        let user_can_delete = verify_role_has_permission(
            &params.staff_role,
            RolePermissions::DeleteComment
        );

        if !user_can_delete && comment.author_id() != params.user_id { return Err(Box::new(UnauthorizedError::new())); }

        let response = self.comment_repository.delete(comment).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Delete Comment Service, while deleting the comment: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }
        
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;
    use tokio;

    use super::{DeleteCommentParams, DeleteCommentService};

    use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::comment::Comment;

    #[tokio::test]
    async fn test() {
        let mut mocked_comment_repo: MockCommentRepositoryTrait = MockCommentRepositoryTrait::new();

        let comment = Comment::new(
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            "Conteúdo inicial".to_string(),
        );

        let comment_db: Arc<Mutex<Vec<Comment>>> = Arc::new(Mutex::new(vec![
            comment.clone()
        ]));
        
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
            comment_repository: Box::new(mocked_comment_repo)
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
