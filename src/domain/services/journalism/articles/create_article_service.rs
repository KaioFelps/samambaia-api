use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{RolePermissions, generate_service_internal_error, verify_role_has_permission};

pub struct CreateArticleParams<'a> {
    pub staff: &'a User,
    pub custom_author_id: Option<Uuid>,
    pub cover_url: String,
    pub title: String,
    pub content: String,
    pub description: String,
    pub tags: Vec<i32>,
    pub script: Option<String>,
}
pub struct CreateArticleService<
    ArticleRepository: ArticleRepositoryTrait,
    ArticleTagRepository: ArticleTagRepositoryTrait,
> {
    article_repository: ArticleRepository,
    article_tag_repository: ArticleTagRepository,
}

impl<ArticleRepository, ArticleTagRepository>
    CreateArticleService<ArticleRepository, ArticleTagRepository>
where
    ArticleRepository: ArticleRepositoryTrait,
    ArticleTagRepository: ArticleTagRepositoryTrait,
{
    pub fn new(
        article_repository: ArticleRepository,
        article_tag_repository: ArticleTagRepository,
    ) -> Self {
        CreateArticleService {
            article_repository,
            article_tag_repository,
        }
    }

    pub async fn exec(&self, params: CreateArticleParams<'_>) -> Result<Article, SamambaiaError> {
        if !verify_role_has_permission(
            &params.staff.role().unwrap(),
            RolePermissions::CreateArticle,
        ) {
            return Err(SamambaiaError::unauthorized_err());
        }

        let author_id = params.custom_author_id.unwrap_or(params.staff.id());

        let tags = self
            .article_tag_repository
            .find_many_by_ids(params.tags)
            .await?;

        let article = Article::new(
            author_id,
            params.title,
            params.content,
            params.cover_url,
            params.description,
            tags,
            params.script,
        );

        self.article_repository
            .create(article)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error ocurred at create article service, while persisting the article",
                    err,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use super::CreateArticleParams;
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;

    #[tokio::test]
    async fn test() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let user = User::new("Kaio".into(), "123".into(), Some(Role::Writer));
        let tag = ArticleTag::new_from_existing(1, "News".into());

        article_tag_repository
            .tag_db
            .lock()
            .unwrap()
            .push(tag.clone());

        let service = super::CreateArticleService {
            article_repository,
            article_tag_repository,
        };

        let result = service
            .exec(CreateArticleParams {
                custom_author_id: None,
                staff: &user,
                content: "Article content right here!".to_string(),
                cover_url: "https://i.url/to/cover".to_string(),
                title: "Fake title".to_string(),
                description: "A humble description...".into(),
                tags: vec![tag.id()],
                script: None,
            })
            .await;

        assert_eq!("Article content right here!", result.unwrap().content());
    }
}
