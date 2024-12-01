use async_trait::async_trait;
use migration::{Expr, Func};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait};
use std::error::Error;

use crate::core::pagination::PaginationParameters;
use crate::domain::domain_entities::comment_report::CommentReport;
use crate::domain::domain_entities::comment_report::CommentReportIdTrait;
use crate::domain::domain_entities::comment_report::DraftCommentReport;
use crate::domain::repositories::comment_report_repository::{
    CommentReportQueryType, CommentReportRepositoryTrait, FindManyCommentReportsResponse,
};
use crate::infra::sea::mappers::sea_comment_report_mapper::SeaCommentReportMapper;
use crate::infra::sea::sea_service::SeaService;

use entities::comment_report::Column as CommentReportColumn;
use entities::comment_report::Entity as CommentReportEntity;

pub struct SeaCommentReportRepository<'a> {
    sea_service: &'a SeaService,
}

impl<'a> SeaCommentReportRepository<'a> {
    // constructor
    pub async fn new(service: &'a SeaService) -> Self {
        SeaCommentReportRepository {
            sea_service: service,
        }
    }
}

#[async_trait]
impl CommentReportRepositoryTrait for SeaCommentReportRepository<'_> {
    async fn create(
        &self,
        comment_report: DraftCommentReport,
    ) -> Result<CommentReport, Box<dyn Error>> {
        let new_comment_report =
            SeaCommentReportMapper::draft_comment_report_to_sea_active_model(comment_report);

        let db = &self.sea_service.db;

        let created_comment_report = new_comment_report.insert(db).await?;
        let created_comment_report =
            SeaCommentReportMapper::model_to_comment_report(created_comment_report);

        Ok(created_comment_report)
    }

    async fn find_by_id(
        &self,
        comm_report_id: i32,
    ) -> Result<Option<CommentReport>, Box<dyn Error>> {
        let comm_report = CommentReportEntity::find_by_id(comm_report_id)
            .one(&self.sea_service.db)
            .await?;

        match comm_report {
            None => Ok(None),
            Some(comm_report) => Ok(Some(SeaCommentReportMapper::model_to_comment_report(
                comm_report,
            ))),
        }
    }

    async fn find_many(
        &self,
        params: PaginationParameters<CommentReportQueryType>,
    ) -> Result<FindManyCommentReportsResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let comment_reports_response = CommentReportEntity::find()
            .order_by_desc(CommentReportColumn::CreatedAt)
            .apply_if(
                params.clone().query,
                |#[allow(unused_mut)] mut query_builder, query| {
                    self.find_many_get_filters(query_builder, query)
                },
            )
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let comment_reports_count = CommentReportEntity::find()
            .apply_if(
                params.query,
                |#[allow(unused_mut)] mut query_builder, query| {
                    self.find_many_get_filters(query_builder, query)
                },
            )
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let mut comment_reports: Vec<CommentReport> = vec![];

        for comm_report in comment_reports_response.into_iter() {
            comment_reports.push(SeaCommentReportMapper::model_to_comment_report(comm_report));
        }

        Ok(FindManyCommentReportsResponse(
            comment_reports,
            comment_reports_count,
        ))
    }

    async fn save(&self, comment_report: CommentReport) -> Result<CommentReport, Box<dyn Error>> {
        let comm_rep_id = comment_report.id();

        let comment_report =
            SeaCommentReportMapper::comment_report_to_sea_active_model(comment_report);

        let comment_report = CommentReportEntity::update(comment_report)
            .filter(CommentReportColumn::Id.eq(comm_rep_id))
            .exec(&self.sea_service.db)
            .await?;

        let comment_report = SeaCommentReportMapper::model_to_comment_report(comment_report);

        Ok(comment_report)
    }

    async fn delete(&self, comment_report: CommentReport) -> Result<(), Box<dyn Error>> {
        let comment_report =
            SeaCommentReportMapper::comment_report_to_sea_active_model(comment_report);

        CommentReportEntity::delete(comment_report)
            .exec(&self.sea_service.db)
            .await?;

        Ok(())
    }
}

impl SeaCommentReportRepository<'_> {
    fn find_many_get_filters(
        &self,
        query_builder: sea_orm::Select<CommentReportEntity>,
        query: CommentReportQueryType,
    ) -> sea_orm::Select<CommentReportEntity> {
        match query {
            CommentReportQueryType::Content(content) => {
                let filter = Expr::expr(Func::lower(Expr::col(CommentReportColumn::Message)))
                    .like(format!("%{}%", content.to_lowercase()));
                query_builder.filter(filter)
            }
            CommentReportQueryType::SolvedBy(id) => {
                let filter = CommentReportColumn::SolvedBy.eq(id);
                query_builder.filter(filter)
            }
            CommentReportQueryType::Solved(solved) => {
                let filter = CommentReportColumn::SolvedBy.is_null().eq(!solved);
                query_builder.filter(filter)
            }
        }
    }
}
