use log::error;
use uuid::Uuid;

use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::comment_repository::CommentRepositoryTrait;
use crate::error::SamambaiaError;
use crate::{LOG_SEP, R_EOL};

pub struct CommentOnArticleParams {
    pub author_id: Uuid,
    pub article_id: Uuid,
    pub content: String,
}

pub struct CommentOnArticleService<CR, AR>
where
    CR: CommentRepositoryTrait,
    AR: ArticleRepositoryTrait,
{
    comment_repository: CR,
    article_repository: AR,
}

impl<CR: CommentRepositoryTrait, AR: ArticleRepositoryTrait> CommentOnArticleService<CR, AR> {
    pub fn new(comment_repository: CR, article_repository: AR) -> Self {
        CommentOnArticleService {
            comment_repository,
            article_repository,
        }
    }

    pub async fn exec(&self, params: CommentOnArticleParams) -> Result<Comment, SamambaiaError> {
        let article_on_db = self.article_repository.find_by_id(params.article_id).await;

        if article_on_db.is_err() {
            error!("{R_EOL}{LOG_SEP}{R_EOL}Error occurred on comment_on_article_service.rs, while fetching article from db:{R_EOL}{:#?}{R_EOL}{LOG_SEP}{R_EOL}", article_on_db.unwrap_err());
            return Err(SamambaiaError::internal_err());
        }

        if article_on_db.unwrap().is_none() {
            return Err(SamambaiaError::bad_request_err());
        }

        let comment = Comment::new(params.author_id, Some(params.article_id), params.content);

        let response = self.comment_repository.create(comment).await;

        if response.is_err() {
            error!("{R_EOL}{LOG_SEP}{R_EOL}Error occurred on comment_on_article_service.rs, while creating comment transaction:{R_EOL}{:#?}{R_EOL}{LOG_SEP}{R_EOL}", response.unwrap_err());
            return Err(SamambaiaError::internal_err());
        }

        Ok(response.unwrap())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;
    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::article_tag::DraftArticleTag;
    use crate::domain::value_objects::slug::Slug;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::libs::time::TimeHelper;
    use crate::tests::relationship_managers::comment_article::CommentArticleRelationInMemoryManager;
    use crate::tests::repositories::article_repository::InMemoryArticleRepository;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;
    use crate::tests::repositories::comment_repository::get_comment_repository;

    #[tokio::test]
    async fn it_should_comment_on_an_article() {
        let article_tags_repository = InMemoryArticleTagRepository::default();
        let mocked_article_repo =
            InMemoryArticleRepository::default(article_tags_repository.clone());

        let comment_article_manager = Arc::new(CommentArticleRelationInMemoryManager::new());
        let (_, mocked_comment_repo) =
            get_comment_repository(None, comment_article_manager.clone());

        let foo_tag = article_tags_repository
            .create(DraftArticleTag::new("Foo".into()))
            .await
            .unwrap();

        let user_id = Uuid::new_v4();
        let article_id = Uuid::new_v4();

        mocked_article_repo
            .article_db
            .lock()
            .unwrap()
            .push(Article::new_from_existing(
                article_id,
                user_id,
                "cover_url".into(),
                "title".into(),
                "content".into(),
                false,
                TimeHelper::now(),
                None,
                Slug::new(article_id, "title".into()),
                "description".into(),
                vec![foo_tag.clone()],
            ));

        let sut = CommentOnArticleService::new(mocked_comment_repo, mocked_article_repo);

        let res = sut
            .exec(CommentOnArticleParams {
                article_id,
                author_id: user_id,
                content: "This article is awesome!".into(),
            })
            .await;

        assert!(res.is_ok());

        let res = res.unwrap();

        assert_eq!(
            (res.author_id(), res.content()),
            (user_id, "This article is awesome!")
        );

        assert_eq!(1, comment_article_manager.db.lock().unwrap().len());

        let relation_1 = comment_article_manager.db.lock().unwrap()[0];

        assert_eq!(
            (relation_1.article_id, relation_1.comment_id),
            (article_id, res.id())
        );
    }
}
