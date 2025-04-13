use std::error::Error;

use async_trait::async_trait;
use uuid::Uuid;

use super::LocalDb;
use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::comment_with_author::CommentWithAuthor;
use crate::domain::repositories::comment_user_article_repository::{
    CommentUserArticleRepositoryTrait,
    CommentWithAuthorQueryType,
    FindManyCommentsWithAuthorResponse,
};

#[derive(Default)]
pub struct InMemoryCommentUserRepository {
    pub comment_with_author_db: LocalDb<CommentWithAuthor>,
}

#[async_trait]
impl CommentUserArticleRepositoryTrait for InMemoryCommentUserRepository {
    async fn find_many_comments(
        &self,
        article_id: Uuid,
        include_inactive: bool,
        params: PaginationParameters<CommentWithAuthorQueryType>,
    ) -> Result<FindManyCommentsWithAuthorResponse, Box<dyn Error>> {
        let PaginationParameters {
            page,
            items_per_page,
            query,
        } = params;

        let comments: Vec<CommentWithAuthor> = match query {
            None => self.comment_with_author_db.lock().unwrap().to_vec(),
            Some(query) => match query {
                CommentWithAuthorQueryType::Content(content) => self
                    .comment_with_author_db
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|item| {
                        item.article_id().is_some_and(|id| id == article_id)
                            || item
                                .content()
                                .to_lowercase()
                                .contains(&content.to_lowercase()[..])
                            || include_inactive
                            || item.is_active()
                    })
                    .cloned()
                    .collect::<Vec<_>>(),
                CommentWithAuthorQueryType::Author(content) => self
                    .comment_with_author_db
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|item| {
                        item.article_id().is_some_and(|id| id == article_id)
                            || item.author().id().eq(&content)
                            || include_inactive
                            || item.is_active()
                    })
                    .cloned()
                    .collect::<Vec<_>>(),
            },
        };

        let total_of_items_before_paginating = comments.len();

        let leap = (page - 1) * items_per_page;

        let mut res_comments: Vec<CommentWithAuthor> = vec![];

        for (index, item) in comments.iter().enumerate() {
            if index >= leap as usize {
                res_comments.push(item.to_owned());
            }
        }

        Ok(FindManyCommentsWithAuthorResponse(
            res_comments,
            total_of_items_before_paginating as u64,
        ))
    }
}
