use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::util::generate_service_internal_error;

pub struct FetchHomePageArticlesService<ArticleRepository: ArticleRepositoryTrait> {
    article_repository: Box<ArticleRepository>
}

impl<ArticleRepository: ArticleRepositoryTrait> FetchHomePageArticlesService<ArticleRepository> {
    pub fn new(article_repository: Box<ArticleRepository>) -> Self {
        FetchHomePageArticlesService {
            article_repository
        }
    }

    pub async fn exec(&self) -> Result<Vec<Article>, Box<dyn DomainErrorTrait>> {
        let articles = self.article_repository.get_home_articles().await;

        if articles.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred on Fetch Home Page Articles Service, while getting the articles from database".into(),
                &articles.unwrap_err()
            ));
        }

        let articles = articles.unwrap();
        Ok(articles)
    }
}

#[cfg(test)]
mod test {
    use tokio;
    use uuid::Uuid;
    use crate::tests::repositories::article_repository::get_article_repository;
    use super::*;

    #[tokio::test]
    async fn test() {
        let (article_db, article_repository) = get_article_repository();

        article_db.lock().unwrap().push(Article::new(Uuid::new_v4(), "Título da notícia 1".to_string(), "Conteúdo da primeira notícia".to_string(), "url".to_string(), 1, "Foo".into()));
        article_db.lock().unwrap().push(Article::new(Uuid::new_v4(), "Título da notícia 2".to_string(), "Conteúdo da segunda notícia".to_string(), "url".to_string(), 1, "Foo".into()));
        article_db.lock().unwrap().push(Article::new(Uuid::new_v4(), "Título da notícia 3".to_string(), "Conteúdo da terceira notícia".to_string(), "url".to_string(), 1, "Foo".into()));

        let service = FetchHomePageArticlesService::new(Box::new(article_repository));

        let result = service.exec().await.unwrap();

        assert_eq!(3, result.len());
    }
}