use async_trait::async_trait;
use uuid::Uuid;
use std::error::Error;

use crate::domain::domain_entities::article::Article;

#[cfg(test)]
use mockall::automock;


#[cfg_attr(test, automock)]
#[async_trait]
pub trait ArticleRepositoryTrait {
    async fn create(&self, article: Article) -> Result<Article, Box<dyn Error>>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>>;

    async fn save(&self, article: Article) -> Result<Article, Box<dyn Error>>;

}

#[cfg(test)]
mod test {
    use tokio;
    use uuid::Uuid;
    use super::*;

    #[tokio::test]
    async fn create() {
        let mut db: Vec<Article> = vec![];
        let mut mocked_repo = MockArticleRepositoryTrait::default();

        mocked_repo
        .expect_create()
        .returning(move |article: Article| {
            db.push(article);

            Ok(db[0].clone())
        });

        let article = Article::new(
            Uuid::new_v4(),
            "Title".to_string(),
            "Content".to_string(),
            "cover-url.com".to_string()
        );

        let result = mocked_repo.create(article).await;

        assert!(!result.unwrap().id().is_nil());
    }
}