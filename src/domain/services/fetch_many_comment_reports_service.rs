use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::{LOG_SEP, R_EOL};
use log::error;
use uuid::Uuid;

use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::repositories::comment_report_repository::{
    CommentReportQueryType, CommentReportRepositoryTrait, FindManyCommentReportsResponse,
};
use crate::errors::internal_error::InternalError;

type Error = Box<dyn DomainErrorTrait>;

pub enum CommentReportServiceQuery {
    /*
     * This should receive a option of the user's NICKNAME.
     * The nickname will be used to get the user's ID that will be, in fact, used to find the related reports.
     */
    SolvedBy(String),
    Solved(bool),
    Content(String),
}

pub struct FetchManyCommentReportsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<CommentReportServiceQuery>,
}

#[derive(Debug)]
pub struct FetchManyCommentReportsResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<CommentReport>,
}

pub struct FetchManyCommentReportsService<
    CommentReportRepository: CommentReportRepositoryTrait,
    UserRepository: UserRepositoryTrait,
> {
    comment_report_repository: Box<CommentReportRepository>,
    user_repository: Box<UserRepository>,
}

impl<
        CommentReportRepository: CommentReportRepositoryTrait,
        UserRepository: UserRepositoryTrait,
    > FetchManyCommentReportsService<CommentReportRepository, UserRepository>
{
    pub fn new(
        comment_report_repository: Box<CommentReportRepository>,
        user_repository: Box<UserRepository>,
    ) -> Self {
        FetchManyCommentReportsService {
            comment_report_repository,
            user_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchManyCommentReportsParams,
    ) -> Result<FetchManyCommentReportsResponse, Error> {
        let default_items_per_page = 9;
        let default_page = 1;

        let items_per_page = if params.per_page.is_some() {
            params.per_page.unwrap()
        } else {
            default_items_per_page
        };

        let page = if params.page.is_some() {
            let params_page = params.page.unwrap();
            if params_page == 0 {
                default_page
            } else {
                params_page
            }
        } else {
            default_page
        };

        let parsed_query = self.parse_query(params.query).await?;

        let response = self
            .comment_report_repository
            .find_many(PaginationParameters {
                items_per_page,
                page,
                query: parsed_query,
            })
            .await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Articles Service, while finding many articles from database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let response = response.unwrap();
        let FindManyCommentReportsResponse(data, total_items) = response;

        Ok(FetchManyCommentReportsResponse {
            pagination: PaginationResponse {
                total_items,
                current_page: page,
                total_pages: (total_items as f64 / items_per_page as f64).ceil() as u32,
            },
            data,
        })
    }

    async fn parse_query(
        &self,
        service_query: Option<CommentReportServiceQuery>,
    ) -> Result<Option<CommentReportQueryType>, Error> {
        if service_query.is_none() {
            return Ok(None);
        }

        match service_query.unwrap() {
            CommentReportServiceQuery::Content(content) => {
                Ok(Some(CommentReportQueryType::Content(content)))
            }
            CommentReportServiceQuery::Solved(value) => {
                Ok(Some(CommentReportQueryType::Solved(value)))
            }
            CommentReportServiceQuery::SolvedBy(nickname) => {
                let user = self.get_id_from_nickname(nickname).await;

                if user.is_err() {
                    error!(
                        "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Articles Service, while parsing the query: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                        user.unwrap_err()
                    );

                    return Err(Box::new(InternalError::new()));
                }

                let user_id = user.unwrap();

                if user_id.is_none() {
                    return Err(Box::new(ResourceNotFoundError::new()));
                }

                let user_id = user_id.unwrap();

                Ok(Some(CommentReportQueryType::SolvedBy(user_id)))
            }
        }
    }

    async fn get_id_from_nickname(
        &self,
        nickname: String,
    ) -> Result<Option<Uuid>, Box<dyn std::error::Error>> {
        let user = self.user_repository.find_by_nickname(&nickname).await?;

        match user {
            None => Ok(None),
            Some(user) => Ok(Some(user.id())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        domain::{
            domain_entities::{comment_report::CommentReportTrait, user::User},
            repositories::{
                comment_report_repository::MockCommentReportRepositoryTrait,
                user_repository::MockUserRepositoryTrait,
            },
        },
        libs::time::TimeHelper,
    };

    use super::*;

    use std::sync::{Arc, Mutex};

    use tokio;
    use uuid::Uuid;

    #[tokio::test]
    async fn test() {
        let comment_report_db: Arc<Mutex<Vec<CommentReport>>> = Arc::new(Mutex::new(Vec::new()));
        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));

        let user = User::new(
            "Floricultor".into(),
            "123".into(),
            Some(crate::domain::domain_entities::role::Role::Principal),
        );

        let comm_rep_1 = CommentReport::new_from_existing(
            1,
            Uuid::new_v4(),
            Uuid::new_v4(),
            "report numero 1".into(),
            None,
            TimeHelper::now(),
        );

        let comm_rep_2 = CommentReport::new_from_existing(
            2,
            Uuid::new_v4(),
            Uuid::new_v4(),
            "report numero 2".into(),
            Some(user.id()),
            TimeHelper::now(),
        );

        comment_report_db.lock().unwrap().push(comm_rep_1);
        comment_report_db.lock().unwrap().push(comm_rep_2);
        user_db.lock().unwrap().push(user);

        let mut mocked_comm_report_repo = MockCommentReportRepositoryTrait::new();
        let mut mocked_user_repo = MockUserRepositoryTrait::new();

        let comm_repo_db_clone = Arc::clone(&comment_report_db);
        mocked_comm_report_repo
            .expect_find_many()
            .returning(move |params| {
                let PaginationParameters {
                    items_per_page,
                    page,
                    query,
                } = params;

                let mut comment_reports: Vec<CommentReport> = vec![];

                if query.is_some() {
                    let query = query.unwrap();

                    match query {
                        CommentReportQueryType::Content(content) => {
                            for item in comm_repo_db_clone.lock().unwrap().iter() {
                                if item.message().contains(&content[..]) {
                                    comment_reports.push(item.clone());
                                }
                            }
                        }
                        CommentReportQueryType::SolvedBy(solved_by) => {
                            for item in comm_repo_db_clone.lock().unwrap().iter() {
                                if item.solved_by().is_some()
                                    && item.solved_by().unwrap().eq(&solved_by)
                                {
                                    comment_reports.push(item.clone());
                                }
                            }
                        }
                        CommentReportQueryType::Solved(solved) => {
                            for item in comm_repo_db_clone.lock().unwrap().iter() {
                                if item.solved_by().is_some().eq(&solved) {
                                    comment_reports.push(item.clone());
                                }
                            }
                        }
                    };
                } else {
                    comment_reports = comm_repo_db_clone.lock().unwrap().to_vec();
                }

                let total_of_items_before_paginating = comment_reports.len();

                let leap = (page - 1) * items_per_page;

                let mut res_comment_reports = vec![];

                for (index, item) in comment_reports.iter().enumerate() {
                    if index >= leap as usize {
                        res_comment_reports.push(item.to_owned());
                    }
                }

                Ok(FindManyCommentReportsResponse(
                    res_comment_reports,
                    total_of_items_before_paginating as u64,
                ))
            });

        let user_repo_db_clone = Arc::clone(&user_db);
        mocked_user_repo
            .expect_find_by_nickname()
            .returning(move |nickname| {
                for user in user_repo_db_clone.lock().unwrap().iter() {
                    if user.nickname().to_lowercase().eq(&nickname.to_lowercase()) {
                        return Ok(Some(user.clone()));
                    }
                }

                Ok(None)
            });

        let sut = FetchManyCommentReportsService {
            comment_report_repository: Box::new(mocked_comm_report_repo),
            user_repository: Box::new(mocked_user_repo),
        };

        let res = sut
            .exec(FetchManyCommentReportsParams {
                page: Some(1),
                per_page: Some(1),
                query: Some(CommentReportServiceQuery::SolvedBy("Floricultor".into())),
            })
            .await;

        let res = res.unwrap();

        assert_eq!("report numero 2".to_string(), res.data[0].message());
        assert_eq!(1, res.pagination.total_pages);
        assert_eq!(1, res.pagination.total_items);
    }
}
