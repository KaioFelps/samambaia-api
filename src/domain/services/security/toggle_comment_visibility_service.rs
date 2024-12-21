use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::error::DomainError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct ToggleCommentVisibilityParams<'exec> {
    pub user_role: &'exec Role,
    pub comment_id: Uuid,
}

pub struct ToggleCommentVisibilityService<CommentRepository: CommentRepositoryTrait> {
    comment_repository: CommentRepository,
}

impl<CommentRepository: CommentRepositoryTrait> ToggleCommentVisibilityService<CommentRepository> {
    pub fn new(comment_repository: CommentRepository) -> Self {
        ToggleCommentVisibilityService { comment_repository }
    }

    pub async fn exec(
        &self,
        params: ToggleCommentVisibilityParams<'_>,
    ) -> Result<Comment, DomainError> {
        let user_can_toggle_visibility =
            verify_role_has_permission(params.user_role, RolePermissions::InactivateComment);

        if !user_can_toggle_visibility {
            return Err(DomainError::unauthorized_err());
        }

        let comment = self
            .comment_repository
            .find_by_id(params.comment_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
            "Error occurred on Toggle Comment Visibility Service, while finding comment by id",
            err,
        )
            })?;

        if comment.is_none() {
            return Err(DomainError::unauthorized_err());
        }

        let mut comment = comment.unwrap();

        if comment.is_active() {
            comment.set_is_active(false);
        } else {
            comment.set_is_active(true);
        }

        self.comment_repository
            .save(comment)
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred on Toggle Comment Visibility Service, while saving the comment on the database",
                err,
            ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::comment_repository::MockCommentRepositoryTrait;

    use std::sync::{Arc, Mutex};
    use tokio;

    #[tokio::test]
    async fn test() {
        // POPULATING THE DATABASE
        let comment_db: Arc<Mutex<Vec<Comment>>> = Arc::new(Mutex::new(vec![]));

        let comment = Comment::new(
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            "Comment content haha".into(),
        );

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
        mocked_comment_repo.expect_save().returning(move |comment| {
            comment_db_clone.lock().unwrap()[0] = comment.clone();

            Ok(comment)
        });

        // SERVICE INSTANTIATING
        let sut = ToggleCommentVisibilityService {
            comment_repository: mocked_comment_repo,
        };

        let res = sut
            .exec(ToggleCommentVisibilityParams {
                user_role: &Role::Editor,
                comment_id: comment.id(),
            })
            .await;

        assert!(res.is_err());
        assert!(comment_db.lock().unwrap()[0].is_active());

        let res = sut
            .exec(ToggleCommentVisibilityParams {
                user_role: &Role::Coord,
                comment_id: comment.id(),
            })
            .await;

        assert!(!res.unwrap().is_active());
    }
}
