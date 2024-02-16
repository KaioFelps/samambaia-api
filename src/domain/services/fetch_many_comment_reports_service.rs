use std::error::Error;
use log::error;
use crate::domain::domain_entities::comment_report::CommentReport;
use crate::{R_EOL, LOG_SEP};

use crate::core::pagination::{PaginationParameters, PaginationResponse};
use crate::domain::repositories::comment_report_repository::{CommentReportQueryType, CommentReportRepositoryTrait, FindManyCommentReportsResponse};
use crate::errors::internal_error::InternalError;

pub struct FetchManyCommentReportsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub query: Option<CommentReportQueryType>    
}


#[derive(Debug)]
pub struct FetchManyCommentReportsResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<CommentReport>
}

pub struct FetchManyCommentReportsService<CommentReportRepository: CommentReportRepositoryTrait> {
    comment_report_repository: Box<CommentReportRepository>
}

impl<CommentReportRepository: CommentReportRepositoryTrait>
FetchManyCommentReportsService<CommentReportRepository> {
    pub fn new(
        comment_report_repository: Box<CommentReportRepository>
    ) -> Self {
        FetchManyCommentReportsService {
            comment_report_repository
        }
    }

    pub async fn exec(&self, params: FetchManyCommentReportsParams) -> Result<FetchManyCommentReportsResponse, Box<dyn Error>> {
        let default_items_per_page = 9;
        let default_page = 1;

        let items_per_page = if params.per_page.is_some() { params.per_page.unwrap() } else { default_items_per_page };
        
        let page = if params.page.is_some() {
            let params_page = params.page.unwrap();
            if params_page <= 0 { default_page } else { params_page }
        } else { default_page };

        let query = params.query;

        let response = self.comment_report_repository.find_many(
            PaginationParameters {
                items_per_page,
                page,
                query,
            }
        ).await;

        if response.is_err() {
            error!(
                "{R_EOL}{LOG_SEP}{R_EOL}Error occurred on Fetch Many Articles Service, while finding many articles from database: {R_EOL}{}{R_EOL}{LOG_SEP}{R_EOL}",
                response.as_ref().unwrap_err()
            );

            return Err(Box::new(InternalError::new()));
        }

        let response = response.unwrap();
        let FindManyCommentReportsResponse (data, total_items) = response;

        Ok(FetchManyCommentReportsResponse {
            pagination: PaginationResponse {
                total_items,
                current_page: page,
                total_pages: (total_items as f64 / items_per_page as f64).ceil() as u32,
            },
            data,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::{domain_entities::comment_report::CommentReportTrait, repositories::comment_report_repository::MockCommentReportRepositoryTrait};

    use super::*;

    use std::sync::{Arc, Mutex};

    use tokio;
    use uuid::Uuid;
    use chrono::Utc;

    #[tokio::test]
    async fn test() {
        let comment_report_db: Arc<Mutex<Vec<CommentReport>>> = Arc::new(Mutex::new(Vec::new()));

        let comm_rep_1 = CommentReport::new_from_existing(
            1,
            Uuid::new_v4(),
            Uuid::new_v4(),
            "report numero 1".into(),
            false,
            Utc::now().naive_utc()
        );

        let comm_rep_2 = CommentReport::new_from_existing(
            2,
            Uuid::new_v4(),
            Uuid::new_v4(),
            "report numero 2".into(),
            true,
            Utc::now().naive_utc()
        );

        comment_report_db.lock().unwrap().push(comm_rep_1);
        comment_report_db.lock().unwrap().push(comm_rep_2);

        let mut mocked_comm_report_repo = MockCommentReportRepositoryTrait::new();

        let comm_repo_db_clone = Arc::clone(&comment_report_db);
        mocked_comm_report_repo
        .expect_find_many()
        .returning(move |params| {
            let PaginationParameters {
                items_per_page,
                page,
                query
            } = params;

            let mut comment_reports: Vec<CommentReport> = vec![];

            if query.is_some() {
                let query = query.unwrap();

                match query {
                    CommentReportQueryType::CONTENT(content) => {
                        for item in comm_repo_db_clone.lock().unwrap().iter() {
                            if item.message().contains(&content[..]) {
                                comment_reports.push(item.clone());
                            }
                        }
                    },
                    CommentReportQueryType::SOLVED(is_solved) => {
                        for item in comm_repo_db_clone.lock().unwrap().iter() {
                            if item.solved().eq(&is_solved) {
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

            Ok(FindManyCommentReportsResponse (res_comment_reports, total_of_items_before_paginating as u64))
        });

        let sut = FetchManyCommentReportsService {
            comment_report_repository: Box::new(mocked_comm_report_repo)
        };

        let res = sut.exec(FetchManyCommentReportsParams {
            page: Some(1),
            per_page: Some(1),
            query: Some(CommentReportQueryType::SOLVED(true)),
        }).await;

        let res = res.unwrap();

        assert_eq!("report numero 2".to_string(), res.data[0].message());
        assert_eq!(1, res.pagination.total_pages);
        assert_eq!(1, res.pagination.total_items);
    }
}