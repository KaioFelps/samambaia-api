use entities::user::Model as UserModel;
use entities::user::ActiveModel as UserActiveModel;
use sea_orm::IntoActiveValue;

use crate::domain::domain_entities::user::User;

use super::sea_role_mapper::SeaRoleMapper;

pub struct SeaUserMapper {}

impl SeaUserMapper {
    pub fn user_to_sea_model(user: User) -> UserModel {
        let role = match user.role() {
            None => None,
            Some(role) => Some(SeaRoleMapper::to_sea(role)),
        };

        let sea_model = UserModel {
            id: user.id(),
            nickname: user.nickname().to_string(),
            password: user.password().to_string(),
            role,
            created_at: user.created_at(),
            last_login: user.last_login()
        };

        sea_model
    }

    pub fn user_to_sea_active_model(user: User) -> UserActiveModel {
        let role = match user.role() {
            None => None,
            Some(role) => Some(SeaRoleMapper::to_sea(role)),
        };

        let sea_active_model = UserActiveModel {
            id: user.id().into_active_value(),
            nickname: user.nickname().to_string().into_active_value(),
            password: user.password().to_string().into_active_value(),
            role: sea_orm::ActiveValue::Set(role),
            created_at: user.created_at().into_active_value(),
            last_login: user.last_login().into_active_value()
        };

        sea_active_model
    }

    pub fn active_model_to_user(active_model_user: UserActiveModel) -> User {
        let role = active_model_user.role.unwrap();

        let role = match role {
            None => None,
            Some(role) => Some(SeaRoleMapper::to_domain(role)),
        };
        
        let user = User::new_from_existing(
            active_model_user.id.unwrap(),
            active_model_user.nickname.unwrap(),
            active_model_user.password.unwrap(),
            active_model_user.created_at.unwrap(),
            active_model_user.last_login.unwrap(),
            role,
        );

        user
    }

    pub fn model_to_user(model_user: UserModel) -> User {
        let role = model_user.role;

        let role = match role {
            None => None,
            Some(role) => Some(SeaRoleMapper::to_domain(role)),
        };

        let user = User::new_from_existing(
            model_user.id.to_owned(),
            model_user.nickname.to_owned(),
            model_user.password.to_owned(),
            model_user.created_at.to_owned(),
            model_user.last_login.to_owned(),
            role,
        );

        user
    }
}