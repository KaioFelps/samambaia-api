use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::DomainError;
use crate::util::generate_service_internal_error;
use crate::util::{verify_role_has_permission, RolePermissions};

pub struct CreateArticleParams {
    pub staff_id: Uuid,
    pub custom_author_id: Option<Uuid>,
    pub cover_url: String,
    pub title: String,
    pub content: String,
    pub tag_id: i32,
}
pub struct CreateArticleService<
    ArticleRepository: ArticleRepositoryTrait,
    ArticleTagRepository: ArticleTagRepositoryTrait,
    UserRepository: UserRepositoryTrait,
> {
    article_repository: Box<ArticleRepository>,
    article_tag_repository: Box<ArticleTagRepository>,
    user_repository: Box<UserRepository>,
}

impl<
        ArticleRepository: ArticleRepositoryTrait,
        ArticleTagRepository: ArticleTagRepositoryTrait,
        UserRepository: UserRepositoryTrait,
    > CreateArticleService<ArticleRepository, ArticleTagRepository, UserRepository>
{
    pub fn new(
        article_repository: Box<ArticleRepository>,
        article_tag_repository: Box<ArticleTagRepository>,
        user_repository: Box<UserRepository>,
    ) -> Self {
        CreateArticleService {
            article_repository,
            article_tag_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: CreateArticleParams) -> Result<Article, DomainError> {
        let staff_on_db = self
            .user_repository
            .find_by_id(&params.staff_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error ocurred at create article service, while finding staff user on the database",
                    err,
                )
            })?;

        if (staff_on_db.is_none())
            || !verify_role_has_permission(
                &staff_on_db.unwrap().role().unwrap(),
                RolePermissions::CreateArticle,
            )
        {
            return Err(DomainError::unauthorized_err());
        }

        let author_id = {
            match params.custom_author_id {
                Some(author_id) => author_id,
                _ => params.staff_id,
            }
        };

        let tag = self.article_tag_repository.find_by_id(params.tag_id).await;

        if let Err(err) = tag {
            return Err(generate_service_internal_error(
                "Error occurred at create article service, while finding tag by id",
                err,
            ));
        }
        let tag = tag.unwrap();

        if tag.is_none() {
            return Err(DomainError::bad_request_err()
                .with_message(format!("Tag with id '{}' not found.", params.tag_id)));
        }

        let tag = tag.unwrap();

        let article = Article::new(
            author_id,
            params.title,
            params.content,
            params.cover_url,
            tag.id(),
            tag.value().to_owned(),
        );

        let response = self.article_repository.create(article).await;

        if response.is_err() {
            let err = response.unwrap_err();
            return Err(generate_service_internal_error(
                "Error ocurred at create article service, while persisting the article",
                err,
            ));
        }

        Ok(response.unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::CreateArticleParams;
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::domain::repositories::article_tag_repository::MockArticleTagRepositoryTrait;
    use crate::domain::{
        domain_entities::{role::Role, user::User},
        repositories::user_repository::MockUserRepositoryTrait,
    };
    use crate::tests::repositories::article_repository::get_article_repository;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test() {
        let (_article_db, mocked_article_repo) = get_article_repository();
        let mut mocked_tag_repo: MockArticleTagRepositoryTrait =
            MockArticleTagRepositoryTrait::new();
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();

        let tag_db: Arc<Mutex<Vec<ArticleTag>>> = Arc::new(Mutex::new(vec![]));
        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![]));

        let user = User::new("Kaio".into(), "123".into(), Some(Role::Writer));
        let tag = ArticleTag::new_from_existing(1, "News".into());

        user_db.lock().unwrap().push(user.clone());
        tag_db.lock().unwrap().push(tag.clone());

        let db = Arc::clone(&tag_db);
        mocked_tag_repo.expect_find_by_id().returning(move |id| {
            let mut found_tag = None;

            for tag in db.lock().unwrap().iter() {
                if tag.id().eq(&id) {
                    found_tag = Some(tag.clone());
                }
            }

            Ok(found_tag)
        });

        let db_clone = Arc::clone(&user_db);
        mocked_user_repo.expect_find_by_id().returning(move |id| {
            for user in db_clone.lock().unwrap().iter() {
                if user.id().eq(id) {
                    return Ok(Some(user.clone()));
                }
            }

            Ok(None)
        });

        let service = super::CreateArticleService {
            article_repository: Box::new(mocked_article_repo),
            article_tag_repository: Box::new(mocked_tag_repo),
            user_repository: Box::new(mocked_user_repo),
        };

        let result = service
            .exec(CreateArticleParams {
                custom_author_id: None,
                staff_id: user.id(),
                content: "Conteúdo do artigo aqui".to_string(),
                cover_url: "https://i.imgur.com/fodase".to_string(),
                title: "Fake title".to_string(),
                tag_id: tag.id(),
            })
            .await;

        assert_eq!("Conteúdo do artigo aqui", result.unwrap().content());
    }
}
