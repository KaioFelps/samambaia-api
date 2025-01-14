use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::error::DomainError;
use crate::infra::http::presenters::home_article::MappedHomeArticle;
use crate::util::generate_service_internal_error;

pub struct FetchHomePageArticlesService<ArticleRepository: ArticleRepositoryTrait> {
    article_repository: ArticleRepository,
}

impl<ArticleRepository: ArticleRepositoryTrait> FetchHomePageArticlesService<ArticleRepository> {
    pub fn new(article_repository: ArticleRepository) -> Self {
        FetchHomePageArticlesService { article_repository }
    }

    pub async fn exec(&self) -> Result<Vec<MappedHomeArticle>, DomainError> {
        self.article_repository
            .get_home_articles()
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Fetch Home Page Articles Service, while getting the articles from database",
                   err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        domain::domain_entities::{article::Article, role::Role, user::User},
        tests::repositories::article_repository::get_article_repository,
    };
    use tokio;

    #[tokio::test]
    async fn test() {
        let (article_db, users_db, article_repository) = get_article_repository();

        let author = User::new("Floricultor".into(), "password".into(), Some(Role::Writer));
        let author_id = author.id();

        users_db.lock().unwrap().push(author);

        article_db.lock().unwrap().push(Article::new(
            author_id,
            "Título da notícia 1".to_string(),
            "Conteúdo da primeira notícia".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
        ));
        article_db.lock().unwrap().push(Article::new(
            author_id,
            "Título da notícia 2".to_string(),
            "Conteúdo da segunda notícia".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
        ));
        article_db.lock().unwrap().push(Article::new(
            author_id,
            "Título da notícia 3".to_string(),
            "Conteúdo da terceira notícia".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
        ));

        let service = FetchHomePageArticlesService::new(article_repository);
        let result = service.exec().await;

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(3, result.len());
        assert_eq!("Floricultor", &result[0].author.nickname);
    }
}
