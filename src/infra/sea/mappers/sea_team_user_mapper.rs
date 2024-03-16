use entities::team_user::Model as TeamUserModel;
use entities::team_user::ActiveModel as TeamUserActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::team_user::TeamUser;

pub struct SeaTeamUserMapper {}

impl SeaTeamUserMapper {
    pub fn team_user_to_sea_model(team_user: TeamUser) -> TeamUserModel {
        let sea_model = TeamUserModel {
            id: team_user.id(),
            team_role_id: team_user.team_role_id(),
            nickname: team_user.nickname().into(),
            user_function: team_user.user_function().into(),
            twitter: team_user.twitter(),
            discord: team_user.discord(),
            created_at: team_user.created_at(),
        };

        sea_model
    }

    pub fn team_user_to_sea_active_model(team_user: TeamUser) -> TeamUserActiveModel {
        let sea_active_model = TeamUserActiveModel {
            id: team_user.id().into_active_value(),
            team_role_id: team_user.team_role_id().into_active_value(),
            nickname: team_user.nickname().to_string().into_active_value(),
            user_function: team_user.user_function().to_string().into_active_value(),
            twitter: team_user.twitter().into_active_value(),
            discord: team_user.discord().into_active_value(),
            created_at: team_user.created_at().into_active_value(),
        };

        sea_active_model
    }

    pub fn active_model_to_team_user(active_model_team_user: TeamUserActiveModel) -> TeamUser {        
        let team_user = TeamUser::new_from_existing(
            active_model_team_user.id.unwrap(),
            active_model_team_user.team_role_id.unwrap(),
            active_model_team_user.nickname.unwrap(),
            active_model_team_user.user_function.unwrap(),
            active_model_team_user.twitter.unwrap(),
            active_model_team_user.discord.unwrap(),
            active_model_team_user.created_at.unwrap(),
        );

        team_user
    }

    pub fn model_to_team_user(model_team_user: TeamUserModel) -> TeamUser {
        let team_user = TeamUser::new_from_existing(
            model_team_user.id.into(),
            model_team_user.team_role_id.into(),
            model_team_user.nickname.into(),
            model_team_user.user_function.into(),
            model_team_user.twitter.into(),
            model_team_user.discord.into(),
            model_team_user.created_at.into(),
        );

        team_user
    }
}