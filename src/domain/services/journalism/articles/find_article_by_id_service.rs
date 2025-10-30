use uuid::Uuid;

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::role::Role;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct FindArticleByIdParams<'a> {
    pub user: Option<&'a User>,
    pub article_id: Uuid,
}

pub struct FindArticleByIdService<AR: ArticleRepositoryTrait> {
    article_repository: AR,
}

impl<AR: ArticleRepositoryTrait> FindArticleByIdService<AR> {
    pub fn new(article_repository: AR) -> Self {
        FindArticleByIdService { article_repository }
    }

    pub async fn exec(
        &self,
        params: FindArticleByIdParams<'_>,
    ) -> Result<Option<Article>, SamambaiaError> {
        let article = self.article_repository
            .find_by_id(params.article_id)
            .await
            .map_err(|err| {
                generate_service_internal_error(
                    "Error occurred in Find Article By ID service, on selecting article on the database",
                    err
                )
            })?;

        match article {
            None => Ok(None),
            Some(article) => {
                let user_can_see_article = params.user.is_some_and(|user| {
                    verify_role_has_permission(
                        user.role().as_ref().unwrap(),
                        RolePermissions::SeeUnapprovedArticle,
                    ) || article.author_id().eq(&user.id()) && user.role().unwrap() != Role::User
                });

                if article.approved() || user_can_see_article {
                    Ok(Some(article))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use uuid::Uuid;

    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;

    static ARTICLE: LazyLock<Article> = LazyLock::new(|| {
        Article::new(
            Uuid::new_v4(),
            "Foo".into(),
            "Content".into(),
            "imgur.com".into(),
            "Description".into(),
            vec![],
            None,
        )
    });

    #[tokio::test]
    async fn anyone_should_find_approved_article_by_id() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository);

        let mut article = ARTICLE.clone();

        article.set_approved(true);

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(article.clone());

        let sut = super::FindArticleByIdService::new(article_repository);

        let response = sut
            .exec(super::FindArticleByIdParams {
                article_id: article.id(),
                user: None,
            })
            .await;

        assert!(response.is_ok());
        assert!(response
            .unwrap()
            .is_some_and(|_article| _article.eq(&article)));
    }

    #[tokio::test]
    async fn normal_users_should_not_find_even_own_unapproved_articles() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository);

        let user = User::new_from_existing(
            ARTICLE.author_id(),
            "Floricultor".into(),
            "".into(),
            TimeHelper::now(),
            None,
            Some(Role::User),
        );

        let article = ARTICLE.clone();
        article_repository.article_db.lock().unwrap().push(article);

        let sut = super::FindArticleByIdService::new(article_repository);

        let response = sut
            .exec(super::FindArticleByIdParams {
                article_id: ARTICLE.id(),
                user: Some(&user),
            })
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_none());
    }

    #[tokio::test]
    async fn staffs_should_find_self_unnaproved_articles_by_id() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository);

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(ARTICLE.clone());

        let user = User::new_from_existing(
            ARTICLE.author_id(),
            "Floricultor".into(),
            "".into(),
            TimeHelper::now(),
            None,
            Some(Role::Writer),
        );

        let sut = super::FindArticleByIdService::new(article_repository);
        let response = sut
            .exec(super::FindArticleByIdParams {
                article_id: ARTICLE.id(),
                user: Some(&user),
            })
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_some());
    }

    #[tokio::test]
    async fn staffs_should_find_unnaproved_articles_by_id() {
        let article_tag_repository = InMemoryArticleTagRepository::default();
        let article_repository = InMemoryArticleRepository::default(article_tag_repository);

        article_repository
            .article_db
            .lock()
            .unwrap()
            .push(ARTICLE.clone());

        let user = User::new("Floricultor".into(), "".into(), Some(Role::Editor));

        let sut = super::FindArticleByIdService::new(article_repository);
        let response = sut
            .exec(super::FindArticleByIdParams {
                article_id: ARTICLE.id(),
                user: Some(&user),
            })
            .await;

        assert!(response.is_ok());
        assert!(response.unwrap().is_some());
    }
}
