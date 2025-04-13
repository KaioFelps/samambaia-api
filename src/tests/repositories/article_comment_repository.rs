use std::sync::{Arc, Mutex};

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::comment::Comment;
use crate::domain::repositories::article_comment_repository::{
    CommentQueryType,
    FindManyCommentsResponse,
    MockArticleCommentRepositoryTrait,
};
use crate::tests::relationship_managers::comment_article::CommentArticleRelationInMemoryManager;

type InMemoryDB<T> = Arc<Mutex<Vec<T>>>;

pub fn get_article_comment_repository(
    article_db: Option<InMemoryDB<Article>>,
    comment_db: Option<InMemoryDB<Comment>>,
    comment_article_relationship_manager: Arc<CommentArticleRelationInMemoryManager>,
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

            let new_relationships = comment_article_relationship_manager
                .db
                .lock()
                .unwrap()
                .iter()
                .filter(|relation| relation.article_id != param_article.id())
                .cloned()
                .collect::<Vec<_>>();

            *comment_article_relationship_manager.db.lock().unwrap() = new_relationships;
            *article_db_clone.lock().unwrap() = new_articles;
            *comment_db_clone.lock().unwrap() = new_comments;

            Ok(())
        });

    let comment_db_clone = comment_db.clone();
    repository.expect_find_many_comments().returning(
        move |_article_id, include_inactive, params| {
            let db = comment_db_clone.lock().unwrap().to_vec();
            let PaginationParameters {
                page,
                items_per_page,
                query,
            } = params;

            let mut comments: Vec<Comment> = Vec::new();

            if query.is_some() {
                match query.unwrap() {
                    CommentQueryType::Content(content) => {
                        for item in db.iter() {
                            if item
                                .content()
                                .to_lowercase()
                                .contains(&content.to_lowercase()[..])
                                && (include_inactive || item.is_active())
                            {
                                comments.push(item.clone());
                            }
                        }
                    }
                    CommentQueryType::Author(content) => {
                        for item in db.iter() {
                            if item.author_id().eq(&content)
                                && (include_inactive || item.is_active())
                            {
                                comments.push(item.clone());
                            }
                        }
                    }
                }
            } else {
                for item in db.iter() {
                    if include_inactive || item.is_active() {
                        comments.push(item.clone());
                    }
                }
            }

            let total_of_items_before_paginating = comments.len();

            let leap = (page - 1) * items_per_page;

            let mut res_comments = vec![];

            for (index, item) in comments.iter().enumerate() {
                if index >= leap as usize {
                    res_comments.push(item.to_owned());
                }
            }

            Ok(FindManyCommentsResponse(
                res_comments,
                total_of_items_before_paginating as u64,
            ))
        },
    );

    (article_db, comment_db, repository)
}
