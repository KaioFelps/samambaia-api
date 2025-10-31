use crate::domain::domain_entities::council_alert::CouncilAlert;
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::council_alerts_repository::CouncilAlertRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{RolePermissions, generate_service_internal_error, verify_role_has_permission};

pub struct GetCouncilAlertsParams<'a> {
    pub user: &'a User,
}

pub struct FindAllCouncilAlertsService<CAR: CouncilAlertRepositoryTrait> {
    council_alert_repository: CAR,
}

impl<CAR: CouncilAlertRepositoryTrait> FindAllCouncilAlertsService<CAR> {
    pub fn new(council_alert_repository: CAR) -> Self {
        FindAllCouncilAlertsService {
            council_alert_repository,
        }
    }

    pub async fn exec(
        &self,
        params: GetCouncilAlertsParams<'_>,
    ) -> Result<Vec<CouncilAlert>, SamambaiaError> {
        let staff_can_see_council_alerts = verify_role_has_permission(
            &params.user.role().unwrap(),
            RolePermissions::SeeCouncilAlerts,
        );

        if !staff_can_see_council_alerts {
            return Err(SamambaiaError::unauthorized_err());
        }

        self.council_alert_repository
            .find_all()
            .await
            .map_err(|error| {
                generate_service_internal_error(
                    "Failed to fetch council alerts on fetching them from database",
                    error,
                )
            })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::council_alert::CouncilAlert;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::services::council::find_all_council_alerts_service::GetCouncilAlertsParams;
    use crate::libs::time::TimeHelper;
    use crate::tests::repositories::council_alerts_repository::InMemoryCouncilAlertRepository;

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_get_council_alerts() {
        let repository = InMemoryCouncilAlertRepository::new();

        // region: --- Seeding
        {
            let mut db_lock = repository.council_alert_db.lock().unwrap();
            db_lock.push(CouncilAlert::new_from_existing(
                1,
                "Alert 1".into(),
                "Content 1".into(),
                true,
                TimeHelper::now(),
            ));
            db_lock.push(CouncilAlert::new_from_existing(
                2,
                "Alert 2".into(),
                "Content 2".into(),
                false,
                TimeHelper::now(),
            ));
            db_lock.push(CouncilAlert::new_from_existing(
                3,
                "Alert 3".into(),
                "Content 3".into(),
                true,
                TimeHelper::now(),
            ));
            db_lock.push(CouncilAlert::new_from_existing(
                4,
                "Alert 4".into(),
                "Content 4".into(),
                false,
                TimeHelper::now(),
            ));
            db_lock.push(CouncilAlert::new_from_existing(
                5,
                "Alert 5".into(),
                "Content 5".into(),
                false,
                TimeHelper::now(),
            ));
        }
        // endregion: --- Seeding

        let staff = super::User::new("JohnDoe".into(), "123".into(), Some(Role::Writer));

        let sut = super::FindAllCouncilAlertsService::new(repository);
        let response = sut.exec(GetCouncilAlertsParams { user: &staff }).await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(5, response.len());
    }
}
