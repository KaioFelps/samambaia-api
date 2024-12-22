use entities::team_role::ActiveModel as TeamRoleActiveModel;
use entities::team_role::Model as TeamRoleModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::team_role::TeamRole;

use super::SeaMapper;

pub struct SeaTeamRoleMapper;

impl SeaMapper<TeamRole, TeamRoleModel, TeamRoleActiveModel> for SeaTeamRoleMapper {
    fn entity_into_model(entity: TeamRole) -> TeamRoleModel {
        TeamRoleModel {
            id: entity.id(),
            title: entity.title().into(),
            description: entity.description().into(),
            created_at: entity.created_at(),
        }
    }

    fn entity_into_active_model(entity: TeamRole) -> TeamRoleActiveModel {
        TeamRoleActiveModel {
            id: entity.id().into_active_value(),
            title: entity.title().to_owned().into_active_value(),
            description: entity.description().to_owned().into_active_value(),
            created_at: entity.created_at().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: TeamRoleActiveModel) -> TeamRole {
        TeamRole::new_from_existing(
            active_model.id.unwrap(),
            active_model.title.unwrap(),
            active_model.description.unwrap(),
            active_model.created_at.unwrap(),
        )
    }

    fn model_into_entity(model: TeamRoleModel) -> TeamRole {
        TeamRole::new_from_existing(model.id, model.title, model.description, model.created_at)
    }
}
