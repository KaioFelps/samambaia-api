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
    pub cleanup_script: Option<String>,
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
        let user_can_create_article = verify_role_has_permission(
            &params.staff.role().unwrap(),
            RolePermissions::CreateArticle,
        );

        let user_can_use_scripts = verify_role_has_permission(
            params.staff.role().as_ref().unwrap(),
            RolePermissions::UseArticleScripts,
        );

        let user_tried_to_use_scripts = params.script.is_some() || params.cleanup_script.is_some();

        if !user_can_create_article || (!user_can_use_scripts && user_tried_to_use_scripts) {
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
            params.cleanup_script,
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
    async fn should_create_an_article_if_valid() {
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
                cleanup_script: None,
            })
            .await;

        assert_eq!("Article content right here!", result.unwrap().content());
    }

    #[tokio::test]
    async fn only_principal_or_higher_should_be_allowed_to_use_scripts() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let admin = User::new("John".into(), "123".into(), Some(Role::Admin));
        let manager = User::new("CardiB".into(), "123".into(), Some(Role::Principal));

        let service = super::CreateArticleService {
            article_repository: article_repository.clone(),
            article_tag_repository,
        };

        let result = service
            .exec(CreateArticleParams {
                custom_author_id: None,
                staff: &admin,
                content: "Article content right here!".to_string(),
                cover_url: "https://i.url/to/cover".to_string(),
                title: "Fake title".to_string(),
                description: "A humble description...".into(),
                tags: vec![],
                script: Some("Foo".into()),
                cleanup_script: None,
            })
            .await;

        assert!(result.is_err());
        assert!(article_repository.article_db.lock().unwrap().is_empty());

        let result = service
            .exec(CreateArticleParams {
                custom_author_id: None,
                staff: &admin,
                content: "Article content right here!".to_string(),
                cover_url: "https://i.url/to/cover".to_string(),
                title: "Fake title".to_string(),
                description: "A humble description...".into(),
                tags: vec![],
                script: None,
                cleanup_script: Some("Foo".into()),
            })
            .await;

        assert!(result.is_err());
        assert!(article_repository.article_db.lock().unwrap().is_empty());

        let result = service
            .exec(CreateArticleParams {
                custom_author_id: None,
                staff: &manager,
                content: "Article content right here!".to_string(),
                cover_url: "https://i.url/to/cover".to_string(),
                title: "Fake title".to_string(),
                description: "A humble description...".into(),
                tags: vec![],
                script: Some("Foo".into()),
                cleanup_script: Some("Bar".into()),
            })
            .await;

        assert!(result.is_ok());
        assert!(!article_repository.article_db.lock().unwrap().is_empty());
    }
}
