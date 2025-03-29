use std::sync::{Arc, Mutex};

use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_comment_repository::MockArticleCommentRepositoryTrait;

type InMemoryDB<T> = Arc<Mutex<Vec<T>>>;

pub fn get_article_comment_repository(
    article_db: Option<InMemoryDB<Article>>,
    comment_db: Option<InMemoryDB<Comment>>,
) -> (
    InMemoryDB<Article>,
    InMemoryDB<Comment>,
    MockArticleCommentRepositoryTrait,
) {
    let mut repository = MockArticleCommentRepositoryTrait::new();

    let article_db = article_db.unwrap_or(Arc::new(Mutex::new(Vec::new())));
    let comment_db = comment_db.unwrap_or(Arc::new(Mutex::new(Vec::new())));

    let article_db_clone = article_db.clone();
    let comment_db_clone = comment_db.clone();
    repository
        .expect_delete_article_and_inactivate_comments()
        .returning(move |param_article| {
            let new_articles = article_db_clone
                .lock()
                .unwrap()
                .iter()
                .filter(|article| article.id().ne(&param_article.id()))
                .cloned()
                .collect::<Vec<_>>();

            let new_comments = comment_db_clone
                .lock()
                .unwrap()
                .iter()
                .cloned()
                .map(|mut comment| {
                    if comment
                        .article_id()
                        .is_some_and(|id| id.eq(&param_article.id()))
                    {
                        comment.remove_associations_and_deactivate();
                        comment
                    } else {
                        comment
                    }
                })
                .collect::<Vec<_>>();

            *article_db_clone.lock().unwrap() = new_articles;
            *comment_db_clone.lock().unwrap() = new_comments;
            Ok(())
        });

    (article_db, comment_db, repository)
}
