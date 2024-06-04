use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::generate_service_internal_error;
use crate::util::{verify_role_has_permission, RolePermissions};

pub struct CreateArticleParams {
    pub staff_id: Uuid,
    pub custom_author_id: Option<Uuid>,
    pub cover_url: String,
    pub title: String,
    pub content: String,
}
pub struct CreateArticleService<
    ArticleRepository: ArticleRepositoryTrait,
    UserRepository: UserRepositoryTrait
> {
    article_repository: Box<ArticleRepository>,
    user_repository: Box<UserRepository>
}

impl<
    ArticleRepository: ArticleRepositoryTrait,
    UserRepository: UserRepositoryTrait,
> CreateArticleService<ArticleRepository, UserRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>, user_repository: Box<UserRepository>) -> Self {
        CreateArticleService {
            article_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: CreateArticleParams) -> Result<Article, Box<dyn DomainErrorTrait>> {
        let staff_on_db = self.user_repository.find_by_id(&params.staff_id).await;

        if staff_on_db.is_err() {
            return Err(generate_service_internal_error(
            "Error ocurred at create article service, while finding staff user on the database",
            staff_on_db.as_ref().unwrap_err()
            ))
        }

        let staff_on_db = staff_on_db.unwrap();

        if (staff_on_db.is_none()) || !verify_role_has_permission(&staff_on_db.unwrap().role().unwrap(), RolePermissions::CreateArticle) {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let author_id = {
            match params.custom_author_id {
                Some(author_id) => author_id,
                _ => params.staff_id
            }
        };

        let article = Article::new(
            author_id,
            params.title,
            params.content,
            params.cover_url
        );

        let response = self.article_repository.create(article).await;

        if response.is_err() {
            let err = response.unwrap_err();
            return Err(generate_service_internal_error(
                "Error ocurred at create article service, while persisting the article",
                &err
            ))
        }
        
        return Ok(response.unwrap());
    }
}


#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::domain::{domain_entities::{user::User, role::Role}, repositories::{article_repository::MockArticleRepositoryTrait, user_repository::MockUserRepositoryTrait}};
    use super::{Article, CreateArticleParams};

    #[tokio::test]
    async fn test() {
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();

        let mut article_db: Vec<Article> = vec![];
        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![]));

        let user = User::new("Kaio".into(), "123".into(), Some(Role::Writter));
        user_db.lock().unwrap().push(user.clone());

        mocked_article_repo
        .expect_create()
        .returning(move |article: Article| {
            article_db.push(article);

            Ok(article_db[0].clone())
        })
        .times(1);

        let db_clone = Arc::clone(&user_db);
        mocked_user_repo
        .expect_find_by_id()
        .returning(move |id| {
            for user in db_clone.lock().unwrap().iter() {
                if user.id().eq(id) {
                    return Ok(Some(user.clone()));
                }
            }

            return Ok(None);
        });

        let service = super::CreateArticleService {
            article_repository: Box::new(mocked_article_repo),
            user_repository: Box::new(mocked_user_repo)
        };

        let result = service.exec(CreateArticleParams {
            custom_author_id: None,
            staff_id: user.id(),
            content: "Conteúdo do artigo aqui".to_string(),
            cover_url: "https://i.imgur.com/fodase".to_string(),
            title: "Fake title".to_string()
        }).await;

        assert_eq!("Conteúdo do artigo aqui", result.unwrap().content());
    }
}
