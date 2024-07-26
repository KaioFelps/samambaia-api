use chrono::NaiveDateTime;
use crate::domain::domain_entities::free_badge::FreeBadge;
use crate::domain::repositories::free_badge_repository::FreeBadgeRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::util::generate_service_internal_error;

pub struct CreateFreeBadgeParams {
    pub code: String,
    pub link: String,
    pub link_is_external: bool,
    pub available_until: Option<NaiveDateTime>,
    pub image: String
}

pub struct CreateFreeBadgeService<FreeBadgeRepository: FreeBadgeRepositoryTrait> {
    free_badge_repository: FreeBadgeRepository
}

impl<FreeBadgeRepository: FreeBadgeRepositoryTrait> CreateFreeBadgeService<FreeBadgeRepository> {
    pub fn new(free_badge_repository: FreeBadgeRepository) -> Self {
        CreateFreeBadgeService {
            free_badge_repository
        }
    }

    pub async fn exec(&self, params: CreateFreeBadgeParams) -> Result<FreeBadge, Box<dyn DomainErrorTrait>> {
        let free_badge = FreeBadge::new(
            params.code,
            params.image,
            params.link,
            params.link_is_external,
            params.available_until,
        );

        let free_badge = self.free_badge_repository.create(free_badge).await;

        if free_badge.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred inside Create Free Badge service on creating the item in the database".into(),
                &free_badge.unwrap_err()
            ));
        }

        Ok(free_badge.unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::services::create_free_badge_service::CreateFreeBadgeParams;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::free_badge_repository::get_free_badge_repository;

    #[tokio::test]
    async fn test_if_can_create_free_badge() {
        let (badges_db, free_badge_repository) = get_free_badge_repository();

        let sut = super::CreateFreeBadgeService::new(free_badge_repository);

        let result = sut.exec(CreateFreeBadgeParams {
            image: "i.imgur.com/".into(),
            code: "KF001".into(),
            link: "www.cosmic.com/news/x".into(),
            link_is_external: false,
            available_until: Some(TimeHelper::now() + chrono::Days::new(3))
        }).await;

        assert!(result.is_ok());
        assert_eq!(1, badges_db.lock().unwrap().len());
        assert_eq!(badges_db.lock().unwrap()[0], result.unwrap());
    }
}
