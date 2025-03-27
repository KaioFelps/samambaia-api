use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct CreateArticleParams<'a> {
    pub staff: &'a User,
    pub custom_author_id: Option<Uuid>,
    pub cover_url: String,
    pub title: String,
    pub content: String,
    pub description: String,
    pub tag_id: Option<i32>,
}
pub struct CreateArticleService<
    ArticleRepository: ArticleRepositoryTrait,
    ArticleTagRepository: ArticleTagRepositoryTrait,
> {
    article_repository: ArticleRepository,
    article_tag_repository: ArticleTagRepository,
}

impl<
        ArticleRepository: ArticleRepositoryTrait,
        ArticleTagRepository: ArticleTagRepositoryTrait,
    > CreateArticleService<ArticleRepository, ArticleTagRepository>
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

        let author_id = {
            match params.custom_author_id {
                Some(author_id) => author_id,
                _ => params.staff.id(),
            }
        };

        let tag = match params.tag_id {
            None => None,
            Some(tag_id) => match self
                .article_tag_repository
                .find_by_id(tag_id)
                .await
                .map_err(|err| {
                    generate_service_internal_error(
                        "Error occurred CreateArticleService when querying tag by id.",
                        err,
                    )
                })? {
                None => {
                    return Err(SamambaiaError::bad_request_err()
                        .with_message(format!("Tag with id '{}' not found.", tag_id)))
                }
                Some(tag) => Some(tag),
            },
        };

        let article = Article::new(
            author_id,
            params.title,
            params.content,
            params.cover_url,
            tag.as_ref().map(|tag| tag.id()),
            tag.as_ref().map(|tag| tag.value().to_owned()),
            params.description,
        );

        let response = self.article_repository.create(article).await;

        if response.is_err() {
            let err = response.unwrap_err();
            return Err(generate_service_internal_error(
                "Error ocurred at create article service, while persisting the article",
                err,
            ));
        }

        Ok(response.unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use super::CreateArticleParams;
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_tag_repository::MockArticleTagRepositoryTrait;
    use crate::tests::repositories::article_repository::get_article_repository;

    #[tokio::test]
    async fn test() {
        let (_article_db, _, mocked_article_repo) = get_article_repository();
        let mut mocked_tag_repo: MockArticleTagRepositoryTrait =
            MockArticleTagRepositoryTrait::new();

        let tag_db: Arc<Mutex<Vec<ArticleTag>>> = Arc::new(Mutex::new(vec![]));

        let user = User::new("Kaio".into(), "123".into(), Some(Role::Writer));
        let tag = ArticleTag::new_from_existing(1, "News".into());

        tag_db.lock().unwrap().push(tag.clone());

        let db = Arc::clone(&tag_db);
        mocked_tag_repo.expect_find_by_id().returning(move |id| {
            let mut found_tag = None;

            for tag in db.lock().unwrap().iter() {
                if tag.id().eq(&id) {
                    found_tag = Some(tag.clone());
                }
            }

            Ok(found_tag)
        });

        let service = super::CreateArticleService {
            article_repository: mocked_article_repo,
            article_tag_repository: mocked_tag_repo,
        };

        let result = service
            .exec(CreateArticleParams {
                custom_author_id: None,
                staff: &user,
                content: "Article content right here!".to_string(),
                cover_url: "https://i.imgur.com/fodase".to_string(),
                title: "Fake title".to_string(),
                tag_id: Some(tag.id()),
                description: "A humble description...".into(),
            })
            .await;

        assert_eq!("Article content right here!", result.unwrap().content());
    }
}
