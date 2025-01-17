use crate::{
    core::pagination::{PaginationParameters, PaginationResponse, DEFAULT_PER_PAGE},
    domain::{
        domain_entities::announcement::Announcement,
        repositories::announcements_repository::{
            AnnouncementQueryType, AnnouncementRepositoryTrait,
        },
    },
    error::SamambaiaError,
    util::generate_service_internal_error,
};

pub struct FetchManyAnnouncementsParams {
    pub per_page: Option<u32>,
    pub page: Option<u32>,
    pub query: Option<String>,
}

pub struct FetchManyAnnouncementsResponse {
    pub data: Vec<Announcement>,
    pub pagination: PaginationResponse,
}

pub struct FetchManyAnnouncementsService<AR: AnnouncementRepositoryTrait> {
    announcements_repository: AR,
}

impl<AR: AnnouncementRepositoryTrait> FetchManyAnnouncementsService<AR> {
    pub fn new(announcements_repository: AR) -> Self {
        FetchManyAnnouncementsService {
            announcements_repository,
        }
    }

    pub async fn exec(
        &self,
        params: FetchManyAnnouncementsParams,
    ) -> Result<FetchManyAnnouncementsResponse, SamambaiaError> {
        let items_per_page = params.per_page.unwrap_or(DEFAULT_PER_PAGE as u32);
        let page = params.page.unwrap_or(1).max(1);
        let query = params.query.map(AnnouncementQueryType::Description);

        let announcements = self
            .announcements_repository
            .find_many(PaginationParameters {
                items_per_page,
                page,
                query,
            })
            .await
            .map_err(|err| generate_service_internal_error(
                "Error occurred in Fetch Many Announcements Service, while fetching announcements from database",
                err
            ))?;

        Ok(FetchManyAnnouncementsResponse {
            data: announcements.0,
            pagination: PaginationResponse::new(page, announcements.1, items_per_page),
        })
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        core::pagination::PaginationResponse, domain::domain_entities::announcement::Announcement,
        tests::repositories::announcements_repository::get_announcements_repository,
    };

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_fetch_many_announcements_service() {
        let (announcements_db, announcements_repository) = get_announcements_repository();

        // region: --- Populating
        let mut db_lock = announcements_db.lock().unwrap();

        db_lock.push(Announcement::new(
            "/foo".into(),
            "fake://img.com/a".into(),
            false,
            Uuid::new_v4(),
            "First image".into(),
        ));

        db_lock.push(Announcement::new(
            "www.github.com".into(),
            "fake://img.com/b".into(),
            true,
            Uuid::new_v4(),
            "Second image".into(),
        ));

        db_lock.push(Announcement::new(
            "www.google.com".into(),
            "fake://img.com/c".into(),
            true,
            Uuid::new_v4(),
            "Third image".into(),
        ));

        db_lock.push(Announcement::new(
            "www.google.com".into(),
            "fake://img.com/c".into(),
            true,
            Uuid::new_v4(),
            "Forth, but with third word".into(),
        ));

        drop(db_lock);
        // endregion: --- Populating

        let sut = super::FetchManyAnnouncementsService::new(announcements_repository);

        // region: --- No Parameters
        let response = sut
            .exec(super::FetchManyAnnouncementsParams {
                page: None,
                per_page: None,
                query: None,
            })
            .await
            .unwrap();

        assert_eq!(
            4,
            response.data.len(),
            "fetched list of announcements should have length 3, but got {}",
            response.data.len()
        );

        assert_eq!(
            "Forth, but with third word",
            response.data[0].description(),
            "list should be sorted descending by created_at field"
        );

        assert_eq!(
            response.pagination,
            PaginationResponse {
                current_page: 1,
                total_items: 4,
                total_pages: 1
            }
        );
        // endregion: --- No Parameters
        // region: --- With Page Query
        let response = sut
            .exec(super::FetchManyAnnouncementsParams {
                page: Some(2),
                per_page: Some(2),
                query: None,
            })
            .await
            .unwrap();

        assert_eq!(
            2,
            response.data.len(),
            "fetched list of announcements should have length 2 when per_page is 2, page is 2 and there is 4 on the database. However, it contains {} announcements in the response",
            response.data.len()
        );

        assert_eq!(
            response.pagination,
            PaginationResponse {
                current_page: 2,
                total_items: 4,
                total_pages: 2
            }
        );
        // endregion: --- With Page Query
        // region: --- With Search Query
        let response = sut
            .exec(super::FetchManyAnnouncementsParams {
                page: None,
                per_page: Some(1),
                query: Some("third".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(
            1,
            response.data.len(),
            "fetched announcements list should contain 1 item only"
        );

        assert_eq!(
            "Forth, but with third word",
            response.data[0].description(),
            "found item should have {} description",
            "Forth, but with third word"
        );

        assert_eq!(
            response.pagination,
            PaginationResponse {
                current_page: 1,
                total_items: 2,
                total_pages: 2
            }
        );
        // endregion: --- With Search Query
    }
}
