use crate::domain::domain_entities::council_alert::{CouncilAlert, CouncilAlertDraft};
use crate::domain::domain_entities::user::User;
use crate::domain::repositories::council_alerts_repository::CouncilAlertRepositoryTrait;
use crate::error::SamambaiaError;
use crate::util::{generate_service_internal_error, verify_role_has_permission, RolePermissions};

pub struct CreateCouncilAlertParams<'a> {
    pub title: String,
    pub content: String,
    pub pinned: bool,
    pub user: &'a User,
}

pub struct CreateCouncilAlertService<CAR: CouncilAlertRepositoryTrait> {
    council_alert_repository: CAR,
}

impl<CAR: CouncilAlertRepositoryTrait> CreateCouncilAlertService<CAR> {
    pub fn new(council_alert_repository: CAR) -> Self {
        Self {
            council_alert_repository,
        }
    }

    pub async fn exec(
        &self,
        params: CreateCouncilAlertParams<'_>,
    ) -> Result<CouncilAlert, SamambaiaError> {
        let staff_can_create_council_alert = verify_role_has_permission(
            &params.user.role().unwrap(),
            RolePermissions::CreateCouncilAlert,
        );

        if !staff_can_create_council_alert {
            return Err(SamambaiaError::unauthorized_err());
        }

        let mut draft_council_alert = CouncilAlertDraft::new(params.title, params.content);
        if params.pinned {
            draft_council_alert.pin();
        }

        self.council_alert_repository
            .create(draft_council_alert)
            .await
            .map_err(|error| {
                generate_service_internal_error(
                    "Error occurred on Create Council Alert Service, while creating council alert",
                    error,
                )
            })
    }
}

#[cfg(test)]
#[allow(clippy::await_holding_lock)]
mod test {
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::tests::repositories::council_alerts_repository::get_council_alerts_repository;

    #[tokio::test]
    async fn test_only_council_members_can_create_council_alerts() {
        let (db, repository) = get_council_alerts_repository();
        let user = User::new("JohnDoe".into(), "123".into(), Some(Role::Admin));

        let sut = super::CreateCouncilAlertService::new(repository);
        let response = sut
            .exec(super::CreateCouncilAlertParams {
                content: "foo".into(),
                title: "bar".into(),
                pinned: false,
                user: &user,
            })
            .await;

        let db_lock = db.lock().unwrap();

        assert!(response.is_err());
        assert!(db_lock.is_empty());
    }

    #[tokio::test]
    async fn test_create_council_alert_service() {
        let (db, repository) = get_council_alerts_repository();

        let staff = User::new("JohnDoe".into(), "123".into(), Some(Role::Principal));

        let sut = super::CreateCouncilAlertService::new(repository);
        let response = sut
            .exec(super::CreateCouncilAlertParams {
                content: "Alert Content".into(),
                title: "My Alert".into(),
                pinned: false,
                user: &staff,
            })
            .await;

        assert!(response.is_ok());

        let council_alert = response.unwrap();
        let db_lock = db.lock().unwrap();

        assert!(!db_lock.is_empty());
        assert!(db_lock
            .first()
            .is_some_and(|alert| alert.eq(&council_alert)));
    }
}
