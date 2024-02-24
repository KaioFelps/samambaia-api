use entities::team_role::Model as TeamRoleModel;
use entities::team_role::ActiveModel as TeamRoleActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::team_role::TeamRole;

pub struct SeaTeamRoleMapper {}

impl SeaTeamRoleMapper {
    pub fn team_role_to_sea_model(team_role: TeamRole) -> TeamRoleModel {
        let sea_model = TeamRoleModel {
            id: team_role.id(),
            title: team_role.title().into(),
            description: team_role.description().into(),
            created_at: team_role.created_at(),
        };

        sea_model
    }

    pub fn team_role_to_sea_active_model(team_role: TeamRole) -> TeamRoleActiveModel {
        let sea_active_model = TeamRoleActiveModel {
            id: team_role.id().into_active_value(),
            title: team_role.title().to_owned().into_active_value(),
            description: team_role.description().to_owned().into_active_value(),
            created_at: team_role.created_at().into_active_value(),
        };

        sea_active_model
    }

    pub fn active_model_to_team_role(active_model_team_role: TeamRoleActiveModel) -> TeamRole {        
        let team_role = TeamRole::new_from_existing(
            active_model_team_role.id.unwrap(),
            active_model_team_role.title.unwrap(),
            active_model_team_role.description.unwrap(),
            active_model_team_role.created_at.unwrap(),
        );

        team_role
    }

    pub fn model_to_team_role(model_team_role: TeamRoleModel) -> TeamRole {
        let team_role = TeamRole::new_from_existing(
            model_team_role.id.into(),
            model_team_role.title.into(),
            model_team_role.description.into(),
            model_team_role.created_at.into(),
        );

        team_role
    }
}