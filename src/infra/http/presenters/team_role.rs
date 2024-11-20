use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::domain_entities::team_role::TeamRole;
use crate::infra::http::presenters::presenter::PresenterTrait;

#[derive(Serialize, Deserialize)]
pub struct MappedTeamRole {
    id: Uuid,
    title: String,
    description: String,
}

pub struct TeamRolePresenter;

impl PresenterTrait<TeamRole, MappedTeamRole> for TeamRolePresenter {
    fn to_http(role: TeamRole) -> MappedTeamRole {
        MappedTeamRole {
            id: role.id(),
            title: role.title().to_owned(),
            description: role.description().to_owned(),
        }
    }
}
