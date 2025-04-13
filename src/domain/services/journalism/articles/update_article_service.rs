use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct UpdateArticleParams<'a> {
    pub user: &'a User,
    pub article_id: Uuid,
    pub cover_url: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub description: Option<String>,
    pub approved: Option<bool>,
    pub author_id: Option<Uuid>,
    pub tags: Option<Vec<i32>>,
}
pub struct UpdateArticleService<
    ArticleRepository: ArticleRepositoryTrait,
    ArticleTagRepository: ArticleTagRepositoryTrait,
> {
    article_repository: ArticleRepository,
    article_tag_repository: ArticleTagRepository,
}

impl<
        ArticleRepository: ArticleRepositoryTrait,
        ArticleTagRepository: ArticleTagRepositoryTrait,
    > UpdateArticleService<ArticleRepository, ArticleTagRepository>
{
    pub fn new(
        article_repository: ArticleRepository,
        article_tag_repository: ArticleTagRepository,
    ) -> Self {
        UpdateArticleService {
            article_repository,
            article_tag_repository,
        }
    }

    async fn maybe_find_tags(
        &self,
        tags: Option<Vec<i32>>,
    ) -> Option<Result<Vec<ArticleTag>, SamambaiaError>> {
        self.article_tag_repository
            .find_many_by_ids(tags?)
            .await
            .into()
    }

    pub async fn exec(&self, params: UpdateArticleParams<'_>) -> Result<Article, SamambaiaError> {
        let (article, new_tags) = tokio::join!(
            self.article_repository.find_by_id(params.article_id),
            self.maybe_find_tags(params.tags)
        );

        // article verifications
        let article = article.map_err(|err| {
            generate_service_internal_error(
                "Error occurred on Update Article Service, while finding article by id",
                err,
            )
        })?;

        let mut article = match article {
            None => return Err(SamambaiaError::resource_not_found_err()),
            Some(article) => article,
        };

        // checks user is allowed to perform the update
        let user_role = params.user.role().unwrap();
        let user_can_update =
            verify_role_has_permission(&user_role, RolePermissions::UpdateArticle);

        let user_can_approve =
            verify_role_has_permission(&user_role, RolePermissions::ApproveArticle);

        let user_can_disapprove =
            verify_role_has_permission(&user_role, RolePermissions::DisapproveArticle);

        if !user_can_approve && params.approved.is_some() {
            return Err(SamambaiaError::unauthorized_err());
        }
        if !user_can_disapprove && params.approved.is_some() && !params.approved.unwrap() {
            return Err(SamambaiaError::unauthorized_err());
        }

        let user_is_author = article.author_id() == params.user.id();

        if !user_can_update && !user_is_author {
            return Err(SamambaiaError::unauthorized_err());
        }

        // if user is author but does no longer belong to the team, he can't delete his own article either.
        if user_is_author && user_role == Role::User {
            return Err(SamambaiaError::unauthorized_err());
        }

        let user_can_change_article_author =
            verify_role_has_permission(&user_role, RolePermissions::ChangeArticleAuthor);

        if !user_can_change_article_author && params.author_id.is_some() {
            return Err(SamambaiaError::unauthorized_err());
        }

        // modifies the article where requested
        if let Some(author_id) = params.author_id {
            article.set_author_id(author_id);
        }

        if let Some(content) = params.content {
            article.set_content(content);
        }

        if let Some(description) = params.description {
            article.set_description(description);
        }

        if let Some(title) = params.title {
            article.set_title(title);
        }

        if let Some(cover_url) = params.cover_url {
            article.set_cover_url(cover_url);
        }

        if let Some(approved) = params.approved {
            article.set_approved(approved);
        }

        if let Some(tags) = new_tags {
            article.set_tags(tags?);
        }

        // ensures that uusers wont modify an article after it has been approved making public a content that actually
        // wouldn't be approved

        article.disapprove_if_touched(params.user);

        self.article_repository
            .save(article)
            .await
            .map_err(|err|
                generate_service_internal_error(
                    "Error occurred in Update Article Service, while saving the article on the database",
                    err,
                ))
    }
}

#[cfg(test)]
mod test {
    use http::StatusCode;
    use uuid::Uuid;

    use super::{Article, UpdateArticleParams};
    use crate::domain::domain_entities::article_tag::{ArticleTag, DraftArticleTag};
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::domain::services::journalism::articles::update_article_service::UpdateArticleService;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::{
        ArticleTagArticle,
        InMemoryArticleTagRepository,
    };

    #[tokio::test]
    async fn unauthorized_users_should_not_update_article() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let tag = ArticleTag::new_from_existing(1, "Foo".into());
        let article_id = Uuid::new_v4();

        article_tag_repository
            .tag_db
            .lock()
            .unwrap()
            .push(tag.clone());

        article_tag_repository
            .article_tag_db
            .lock()
            .unwrap()
            .push(ArticleTagArticle {
                article_id,
                article_tag_id: tag.id(),
            });

        let article = Article::new(
            article_id,
            "Initial title".to_string(),
            "Initial content".to_string(),
            "initial.coverurl".to_string(),
            "Initial description".into(),
            vec![tag.clone()],
        );

        let article_tag = ArticleTag::new_from_existing(2, "Bar".to_string());

        article_tag_repository
            .tag_db
            .lock()
            .unwrap()
            .push(article_tag);
        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(article.clone());

        let service = super::UpdateArticleService {
            article_repository,
            article_tag_repository,
        };

        let writer = User::new("John".into(), "".into(), Some(Role::Writer));

        let result = service
            .exec(UpdateArticleParams {
                user: &writer,
                article_id: article.id(),
                approved: Some(true),
                title: None,
                content: None,
                description: None,
                cover_url: None,
                author_id: None,
                tags: Some(Vec::new()),
            })
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().get_code(), StatusCode::UNAUTHORIZED); // writer can't approve any article
    }

    #[tokio::test]
    async fn writer_or_below_should_get_article_disapproved_on_edit() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let writer = User::new("John".into(), "".into(), Some(Role::Writer));

        let mut article = Article::new(
            writer.id(),
            "Title".into(),
            "<h1>Contentm</h1>".into(),
            "url".into(),
            "description".into(),
            Vec::new(),
        );
        article.set_approved(true);

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(article.clone());

        let sut = UpdateArticleService::new(article_repository, article_tag_repository);
        let result = sut
            .exec(UpdateArticleParams {
                content: Some("<h1>Edited Content</h1>".into()),
                cover_url: None,
                description: None,
                title: None,
                approved: None,
                article_id: article.id(),
                author_id: None,
                tags: None,
                user: &writer,
            })
            .await;

        assert!(result.is_ok());
        assert!(!result.unwrap().approved());
    }

    #[tokio::test]
    async fn authorized_user_should_be_able_to_update_article_and_keep_approved() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let editor = User::new("John".into(), "".into(), Some(Role::Editor));

        let mut article = Article::new(
            Uuid::new_v4(),
            "Title".into(),
            "<h1>Contentm</h1>".into(),
            "url".into(),
            "description".into(),
            Vec::new(),
        );
        article.set_approved(true);

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(article.clone());

        let tag = article_tag_repository
            .create(DraftArticleTag::new("Bar".into()))
            .await
            .unwrap();

        article_tag_repository
            .article_tag_db
            .lock()
            .unwrap()
            .push(ArticleTagArticle {
                article_id: article.id(),
                article_tag_id: tag.id(),
            });

        let sut = UpdateArticleService::new(article_repository, article_tag_repository);

        let result = sut
            .exec(UpdateArticleParams {
                user: &editor,
                article_id: article.id(),
                approved: None,
                title: Some("updated title".to_string()),
                content: Some("updated content".to_string()),
                description: Some("updated description".to_string()),
                cover_url: None,
                author_id: None,
                tags: Some(vec![tag.id()]),
            })
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!("updated title", result.title());
        assert_eq!("updated description", result.description());
        assert_eq!("updated content", result.content());

        assert!(!result.get_tags().is_empty());
        assert_eq!("Bar", result.get_tags().first().unwrap().value());
    }

    #[tokio::test]
    async fn articles_tags_should_be_correctly_removed_or_added() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository.clone());

        let editor = User::new("John".into(), "".into(), Some(Role::Editor));

        let tag_1 = article_tag_repository
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();
        let tag_2 = article_tag_repository
            .create(DraftArticleTag::new("Bar".into()))
            .await
            .unwrap();
        let tag_3 = article_tag_repository
            .create(DraftArticleTag::new("Baz".into()))
            .await
            .unwrap();

        let mut article = Article::new(
            Uuid::new_v4(),
            "Title".into(),
            "<h1>Contentm</h1>".into(),
            "url".into(),
            "description".into(),
            vec![tag_1.clone(), tag_2.clone()],
        );

        article.set_approved(true);

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(article.clone());

        let sut = UpdateArticleService::new(article_repository, article_tag_repository);

        let result = sut
            .exec(UpdateArticleParams {
                user: &editor,
                article_id: article.id(),
                approved: None,
                title: Some("updated title".to_string()),
                content: Some("updated content".to_string()),
                description: Some("updated description".to_string()),
                cover_url: None,
                author_id: None,
                tags: Some(vec![tag_3.id()]),
            })
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!("updated title", result.title());
        assert_eq!("updated description", result.description());
        assert_eq!("updated content", result.content());

        assert!(!result.get_tags().is_empty());
        assert!([tag_3].iter().all(|tag| result.get_tags().contains(&tag)));
    }
}
