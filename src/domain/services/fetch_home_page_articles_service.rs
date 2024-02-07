use std::error::Error;
use crate::{domain::{domain_entities::article::Article, repositories::article_repository::ArticleRepositoryTrait}, errors::internal_error::InternalError};

pub struct FetchHomePageArticlesService<ArticleRepository: ArticleRepositoryTrait> {
    article_repository: Box<ArticleRepository>
}

impl<ArticleRepository: ArticleRepositoryTrait> FetchHomePageArticlesService<ArticleRepository> {
    pub fn new(article_repository: Box<ArticleRepository>) -> Self {
        FetchHomePageArticlesService {
            article_repository
        }
    }

    pub async fn exec(&self) -> Result<Vec<Article>, Box<dyn Error>> {
        let articles = self.article_repository.get_home_articles().await;

        if articles.is_err() {
            return Err(Box::new(InternalError::new()));
        }

        let articles = articles.unwrap();
        Ok(articles)
    }
}

#[cfg(test)]
mod test {
    use tokio;
    use uuid::Uuid;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;

    use super::*;

    #[tokio::test]
    async fn test() {
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        mocked_article_repo
        .expect_get_home_articles()
        .returning(|| {
            let articles = vec![
                Article::new(Uuid::new_v4(), "Título da notícia 1".to_string(), "Conteúdo da primeira notícia".to_string(), "url".to_string()),
                Article::new(Uuid::new_v4(), "Título da notícia 2".to_string(), "Conteúdo da segunda notícia".to_string(), "url".to_string()),
                Article::new(Uuid::new_v4(), "Título da notícia 3".to_string(), "Conteúdo da terceira notícia".to_string(), "url".to_string()),
            ];

            Ok(articles)
        });

        let service = FetchHomePageArticlesService::new(Box::new(mocked_article_repo));

        let result = service.exec().await.unwrap();

        assert_eq!(3, result.len());
    }
}