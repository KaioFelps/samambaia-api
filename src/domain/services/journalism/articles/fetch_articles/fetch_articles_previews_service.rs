use super::params::FetchArticlesParams;
use crate::core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE};
use crate::domain::domain_entities::article_preview::ArticlePreview;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub struct FetchArticlesPreviewsService<ArticleRepository, UserRepository>
where
    ArticleRepository: ArticleRepositoryTrait,
    UserRepository: UserRepositoryTrait,
{
    article_repository: ArticleRepository,
    user_repository: UserRepository,
}

pub struct FetchArticlesPreviewsResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<ArticlePreview>,
}

impl<AR, UR> FetchArticlesPreviewsService<AR, UR>
where
    AR: ArticleRepositoryTrait,
    UR: UserRepositoryTrait,
{
    pub fn new(article_repository: AR, user_repository: UR) -> Self {
        FetchArticlesPreviewsService {
            article_repository,
            user_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchArticlesParams,
    ) -> Result<FetchArticlesPreviewsResponse, SamambaiaError> {
        let items_per_page = params.per_page.unwrap_or(DEFAULT_PER_PAGE as u32);
        let page = params.page.unwrap_or(1).max(1);

        let query = super::parse_article_fetch_query(&self.user_repository, params.query).await?;

        self.article_repository
            .find_many_previews(PaginationParameters { page, items_per_page, query }, params.approved_state)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Fetch Articles Previews Service, while getting the articles from database",
                   err,
                )
            })
            .map(|articles| {
                FetchArticlesPreviewsResponse {
                    data: articles.0,
                    pagination: PaginationResponse::new(page, articles.1, items_per_page)
                }
            })
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;
    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::tests::repositories::article_repository::get_article_repository;
    use crate::tests::repositories::users_repository::get_user_repository;

    #[tokio::test]
    async fn test() {
        let (article_db, users_db, article_repository) = get_article_repository();
        let (users_db, user_repository) = get_user_repository(Some(users_db));

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
            "Descrição da notícia 1".into(),
        ));
        article_db.lock().unwrap().push(Article::new(
            author_id,
            "Título da notícia 2".to_string(),
            "Conteúdo da segunda notícia".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
            "Descrição da notícia 2".into(),
        ));
        article_db.lock().unwrap().push(Article::new(
            author_id,
            "Título da notícia 3".to_string(),
            "Conteúdo da terceira notícia".to_string(),
            "url".to_string(),
            1,
            "Foo".into(),
            "Descrição da notícia 3".into(),
        ));

        let service = FetchArticlesPreviewsService::new(article_repository, user_repository);
        let result = service
            .exec(FetchArticlesParams {
                page: None,
                per_page: None,
                query: None,
                approved_state: None,
            })
            .await;

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(3, result.pagination.total_items);
        assert_eq!("Floricultor", result.data[0].author().nickname());
    }
}
