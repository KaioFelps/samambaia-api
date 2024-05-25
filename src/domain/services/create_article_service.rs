use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::internal_error::InternalError;
use crate::{LOG_SEP, R_EOL};

pub struct CreateArticleParams {
    pub author_id: Uuid,
    pub cover_url: String,
    pub title: String,
    pub content: String,
}
pub struct CreateArticleService<
ArticleRepository: ArticleRepositoryTrait,
> {
    article_repository: Box<ArticleRepository>,
}

impl<ArticleRepository: ArticleRepositoryTrait> CreateArticleService<ArticleRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>) -> Self {
        CreateArticleService {
            article_repository,
        }
    }

    pub async fn exec(&self, params: CreateArticleParams) -> Result<Article, Box<dyn DomainErrorTrait>> {
        let article = Article::new(
            params.author_id,
            params.title,
            params.content,
            params.cover_url
        );

        let response = self.article_repository.create(article).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Create Article Service, while creating the article:{R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );
            return Err(Box::new(InternalError::new()));
        }
        
        return Ok(response.unwrap());
    }
}


#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use super::{Article, CreateArticleParams};

    #[tokio::test]
    async fn test() {
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        let mut db: Vec<Article> = vec![];

        mocked_article_repo
        .expect_create()
        .returning(move |article: Article| {
            db.push(article);

            Ok(db[0].clone())
        })
        .times(1);

        let service = super::CreateArticleService {
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
