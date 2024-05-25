use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::{verify_role_has_permission, RolePermissions};

use crate::{LOG_SEP, R_EOL};

pub struct ToggleCommentVisibilityParams {
    pub user_role: Role,
    pub comment_id: Uuid
}

pub struct ToggleCommentVisibilityService <CommentRepository: CommentRepositoryTrait> {
    comment_repository: Box<CommentRepository>
}

impl<CommentRepository: CommentRepositoryTrait>
ToggleCommentVisibilityService<CommentRepository> {
    pub fn new(
        comment_repository: Box<CommentRepository>
    ) -> Self {
        ToggleCommentVisibilityService {
            comment_repository
        }
    }

    pub async fn exec(&self, params: ToggleCommentVisibilityParams) -> Result<Comment, Box<dyn DomainErrorTrait>> {
        let user_can_toggle_visibility = verify_role_has_permission(&params.user_role, RolePermissions::InactivateComment);

        if !user_can_toggle_visibility {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let comment = self.comment_repository.find_by_id(params.comment_id).await;

        if comment.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Toggle Comment Visibility Service, while finding comment by id: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                comment.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let comment = comment.unwrap();

        if comment.is_none() { return Err(Box::new(UnauthorizedError::new())) }

        let mut comment = comment.unwrap();

        if comment.is_active() {
            comment.set_is_active(false);
        } else {
            comment.set_is_active(true);
        }

        let result = self.comment_repository.save(comment).await;

        match result {
            Ok(comment) => Ok(comment),
            Err(err) => {
                error!(
                    "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Toggle Comment Visibility Service, while saving the comment on the database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                    err
                );

                return Err(Box::new(InternalError::new()));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;

    use tokio;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        // POPULATING THE DATABASE
        let comment_db: Arc<Mutex<Vec<Comment>>> = Arc::new(Mutex::new(vec![]));

        let comment = Comment::new(Uuid::new_v4(), Some(Uuid::new_v4()), "Comment content haha".into());

        comment_db.lock().unwrap().push(comment.clone());

        // MOCKED REPOSITORIES
        let mut mocked_comment_repo = MockCommentRepositoryTrait::new();

        let comment_db_clone = Arc::clone(&comment_db);
        mocked_comment_repo
        .expect_find_by_id()
        .returning(move |id| {
            let mut comment = None;
            
            for item in comment_db_clone.lock().unwrap().iter() {
                if item.id().eq(&id) {
                    comment = Some(item.clone());
                    break;
                }
            }
            
            Ok(comment)
        });
        
        let comment_db_clone = Arc::clone(&comment_db);
        mocked_comment_repo
        .expect_save()
        .returning(move |comment| {    
            comment_db_clone.lock().unwrap()[0] = comment.clone();

            Ok(comment)
        });

        // SERVICE INSTANCIATING
        let sut = ToggleCommentVisibilityService {
            comment_repository: Box::new(mocked_comment_repo)
        };

        let res = sut.exec(ToggleCommentVisibilityParams {
            user_role: Role::Editor,
            comment_id: comment.id(),
        }).await;

        assert_eq!(true, res.is_err());
        assert_eq!(true, comment_db.lock().unwrap()[0].is_active());

        let res = sut.exec(ToggleCommentVisibilityParams {
            user_role: Role::Coord,
            comment_id: comment.id(),
        }).await;

        assert_eq!(false, res.unwrap().is_active());
    }
}