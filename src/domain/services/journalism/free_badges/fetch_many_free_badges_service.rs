use crate::core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE};
use crate::domain::domain_entities::free_badge::FreeBadge;
use crate::domain::repositories::free_badge_repository::{
    FindManyFreeBadgesResponse, FreeBadgeRepositoryTrait,
};
use crate::error::SamambaiaError;
use crate::util::generate_service_internal_error;

pub struct FetchManyFreeBadgesParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug)]
pub struct FetchManyFreeBadgesResponse {
    pub pagination: PaginationResponse,
    pub data: Vec<FreeBadge>,
}

pub struct FetchManyFreeBadgesService<FreeBadgeRepository: FreeBadgeRepositoryTrait> {
    free_badge_repository: FreeBadgeRepository,
}

impl<FreeBadgeRepository: FreeBadgeRepositoryTrait>
    FetchManyFreeBadgesService<FreeBadgeRepository>
{
    pub fn new(free_badge_repository: FreeBadgeRepository) -> Self {
        FetchManyFreeBadgesService {
            free_badge_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchManyFreeBadgesParams,
    ) -> Result<FetchManyFreeBadgesResponse, SamambaiaError> {
        let page = params.page.unwrap_or(1);
        let items_per_page = params.per_page.unwrap_or(DEFAULT_PER_PAGE as u32);

        let result = self
            .free_badge_repository
            .find_many(PaginationParameters {
                page,
                items_per_page,
                query: None,
            })
            .await;

        if let Err(err) = result {
            return Err(generate_service_internal_error(
                "Error occurred in Fetch Many Free Badges service, on fetching many free badges from database",
                err,
            ));
        }

        let FindManyFreeBadgesResponse(free_badges, total_items) = result.unwrap();

        Ok(FetchManyFreeBadgesResponse {
            data: free_badges,
            pagination: PaginationResponse::new(page, total_items, items_per_page),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::free_badge::FreeBadge;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::free_badge_repository::get_free_badge_repository;

    #[tokio::test]
    async fn test_if_can_get_paginated_free_badges_list() {
        let (badges_db, badges_repository) = get_free_badge_repository();

        *badges_db.lock().unwrap() = vec![
            FreeBadge::new(
                "PT001".to_string(),
                "i.imgur.com".to_string(),
                "www.cosmic.com/news/1".to_string(),
                false,
                None,
            ),
            FreeBadge::new(
                "PT002".to_string(),
                "i.imgur.com".to_string(),
                "www.cosmic.com/news/2".to_string(),
                false,
                Some(TimeHelper::now() + chrono::Days::new(2)),
            ),
            FreeBadge::new(
                "PT003".to_string(),
                "i.imgur.com".to_string(),
                "www.cosmichotel.com/".to_string(),
                true,
                Some(TimeHelper::now() + chrono::Days::new(4)),
            ),
            FreeBadge::new(
                "PT004".to_string(),
                "i.imgur.com".to_string(),
                "www.cosmic.com/news/10".to_string(),
                false,
                Some(TimeHelper::now() + chrono::Days::new(3)),
            ),
        ];

        let sut = super::FetchManyFreeBadgesService::new(badges_repository);

        let result = sut
            .exec(super::FetchManyFreeBadgesParams {
                per_page: Some(3),
                page: None,
            })
            .await;

        assert!(
            result.is_ok(),
            "Expected service to have been successfully performed."
        );

        let result = result.unwrap();

        assert_eq!(
            result.pagination.total_items, 4,
            "Expected it to be 4 items on the mocked database."
        );
        assert_eq!(
            result.pagination.total_pages, 2,
            "Expected response to have two pages."
        );
        assert_eq!(
            result.pagination.current_page, 1,
            "Expected current page to be 1 by default."
        );
        assert_eq!(result.data.len(), 3, "Expected data to have length 3, since it's what have been passed to the service parameters.");
    }
}
