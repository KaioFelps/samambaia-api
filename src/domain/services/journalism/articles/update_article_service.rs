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
    pub tag_id: Option<i32>,
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

    pub async fn exec(&self, params: UpdateArticleParams<'_>) -> Result<Article, SamambaiaError> {
        // article verifications
        let article = self
            .article_repository
            .find_by_id(params.article_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred on Update Article Service, while finding article by id",
                    err,
                )
            })?;

        let mut article = match article {
            None => return Err(SamambaiaError::resource_not_found_err()),
            Some(article) => article,
        };

        // skips it if there's nothing to be updated
        if params.cover_url.is_none()
            && params.title.is_none()
            && params.approved.is_none()
            && params.author_id.is_none()
            && params.content.is_none()
            && params.description.is_none()
            && params.tag_id.is_none()
        {
            return Ok(article);
        }

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

        if let Some(tag_id) = params.tag_id {
            let tag = self.get_tag_by_id(tag_id).await?;

            article.set_tag_id(tag.id());
            article.set_tag_value(tag.value().to_owned());
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

    async fn get_tag_by_id(&self, tag_id: i32) -> Result<ArticleTag, SamambaiaError> {
        let tag = self
            .article_tag_repository
            .find_by_id(tag_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Update Article Service, while finding article tag by id",
                    err,
                )
            })?;

        if tag.is_none() {
            return Err(SamambaiaError::bad_request_err()
                .with_message(format!("Tag with id '{}' not found.", tag_id)));
        }

        Ok(tag.unwrap())
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
    use crate::tests::repositories::article_repository::get_article_repository;
    use crate::tests::repositories::article_tag_repository::get_article_tag_repository;

    #[tokio::test]
    async fn unauthorized_users_should_not_update_article() {
        let (article_db, _, article_repository) = get_article_repository();
        let (tag_db, article_tag_repository) = get_article_tag_repository();

        let article = Article::new(
            Uuid::new_v4(),
            "Initial title".to_string(),
            "Initial content".to_string(),
            "initial.coverurl".to_string(),
            Some(1),
            Some("Foo".to_string()),
            "Initial description".into(),
        );

        let article_tag = ArticleTag::new_from_existing(2, "Bar".to_string());

        tag_db.lock().unwrap().push(article_tag);
        article_db.lock().unwrap().push(article.clone());

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
                tag_id: None,
            })
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().get_code(), StatusCode::UNAUTHORIZED); // writer can't approve any article
    }

    #[tokio::test]
    async fn writer_or_below_should_get_article_disapproved_on_edit() {
        let (article_db, _, article_repository) = get_article_repository();
        let (_tag_db, article_tag_repository) = get_article_tag_repository();

        let writer = User::new("John".into(), "".into(), Some(Role::Writer));

        let mut article = Article::new(
            writer.id(),
            "Title".into(),
            "<h1>Contentm</h1>".into(),
            "url".into(),
            None,
            None,
            "description".into(),
        );
        article.set_approved(true);

        article_db.lock().unwrap().push(article.clone());

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
                tag_id: None,
                user: &writer,
            })
            .await;

        assert!(result.is_ok());
        assert!(!result.unwrap().approved());
    }

    #[tokio::test]
    async fn authorized_user_should_be_able_to_update_article_and_keep_approved() {
        let (article_db, _, article_repository) = get_article_repository();
        let (_tag_db, article_tag_repository) = get_article_tag_repository();

        let editor = User::new("John".into(), "".into(), Some(Role::Editor));

        let mut article = Article::new(
            Uuid::new_v4(),
            "Title".into(),
            "<h1>Contentm</h1>".into(),
            "url".into(),
            None,
            None,
            "description".into(),
        );
        article.set_approved(true);

        article_db.lock().unwrap().push(article.clone());

        let tag = article_tag_repository
            .create(DraftArticleTag::new("Bar".into()))
            .await
            .unwrap();

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
                tag_id: Some(tag.id()),
            })
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!("updated title", result.title());
        assert_eq!("updated description", result.description());
        assert_eq!("updated content", result.content());
        assert_eq!("Bar", result.tag_value().as_ref().unwrap());
    }
}
