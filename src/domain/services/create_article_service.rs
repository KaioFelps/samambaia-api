use std::error::Error;
use uuid::Uuid;

use crate::domain::domain_entities::{article::Article, role::Role};
use crate::domain::repositories::{article_repository::ArticleRepositoryTrait, user_repository::UserRepositoryTrait};
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};

pub struct CreateArticleParams {
    pub author_id: Uuid,
    pub cover_url: String,
    pub title: String,
    pub content: String,
}
pub struct CreateArticleService<
ArticleRepository: ArticleRepositoryTrait,
UserRepository: UserRepositoryTrait
> {
    user_repository: Box<UserRepository>,
    article_repository: Box<ArticleRepository>,
}

impl
<ArticleRepository: ArticleRepositoryTrait,
UserRepository: UserRepositoryTrait>
CreateArticleService<ArticleRepository, UserRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>, user_repository: Box<UserRepository>) -> Self {
        CreateArticleService {
            article_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: CreateArticleParams) -> Result<Article, Box<dyn Error>> {
        let user_on_db = &self.user_repository.find_by_id(&params.author_id).await;

        if user_on_db.is_err() {
            return Err(
                Box::new(InternalError::new())
            );
        }

        let user_on_db = user_on_db.as_ref().unwrap().to_owned();

        if user_on_db.is_none() || user_on_db.unwrap().role().unwrap() == Role::User {
            return Err(
                Box::new(UnauthorizedError::new())
            )
        }

        let article = Article::new(
            params.author_id,
            params.title,
            params.content,
            params.cover_url
        );

        let response = &self.article_repository.create(article).await;

        if response.as_ref().is_ok() {
            return Ok(response.as_ref().unwrap().clone());
        }

        else {
           return Err(Box::new(InternalError::new()));
        }
    }
}


#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;
    use super::{Article, CreateArticleParams};

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        let mut db: Vec<Article> = vec![];

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

        mocked_article_repo
        .expect_create()
        .returning(move |article: Article| {
            db.push(article);

            Ok(db[0].clone())
        })
        .times(1);

        let service = super::CreateArticleService {
            user_repository: Box::new(mocked_user_repo),
            article_repository: Box::new(mocked_article_repo)
        };

        let result = service.exec(CreateArticleParams {
            author_id: Uuid::new_v4(),
            content: "Conteúdo do artigo aqui".to_string(),
            cover_url: "https://i.imgur.com/fodase".to_string(),
            title: "Fake title".to_string()
        }).await;

        assert_eq!("Conteúdo do artigo aqui", result.unwrap().content());
    }
}
