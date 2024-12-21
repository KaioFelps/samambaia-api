use entities::team_user::ActiveModel as TeamUserActiveModel;
use entities::team_user::Model as TeamUserModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::team_user::TeamUser;

use super::SeaMapper;

pub struct SeaTeamUserMapper;

impl SeaMapper<TeamUser, TeamUserModel, TeamUserActiveModel> for SeaTeamUserMapper {
    fn entity_into_model(entity: TeamUser) -> TeamUserModel {
        TeamUserModel {
            id: entity.id(),
            team_role_id: entity.team_role_id(),
            nickname: entity.nickname().into(),
            user_function: entity.user_function().into(),
            twitter: entity.twitter(),
            discord: entity.discord(),
            created_at: entity.created_at(),
        }
    }

    fn entity_into_active_model(entity: TeamUser) -> TeamUserActiveModel {
        TeamUserActiveModel {
            id: entity.id().into_active_value(),
            team_role_id: entity.team_role_id().into_active_value(),
            nickname: entity.nickname().to_string().into_active_value(),
            user_function: entity.user_function().to_string().into_active_value(),
            twitter: entity.twitter().into_active_value(),
            discord: entity.discord().into_active_value(),
            created_at: entity.created_at().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: TeamUserActiveModel) -> TeamUser {
        TeamUser::new_from_existing(
            active_model.id.unwrap(),
            active_model.team_role_id.unwrap(),
            active_model.nickname.unwrap(),
            active_model.user_function.unwrap(),
            active_model.twitter.unwrap(),
            active_model.discord.unwrap(),
            active_model.created_at.unwrap(),
        )
    }

    fn model_into_entity(model: TeamUserModel) -> TeamUser {
        TeamUser::new_from_existing(
            model.id,
            model.team_role_id,
            model.nickname,
            model.user_function,
            model.twitter,
            model.discord,
            model.created_at,
        )
    }
}
