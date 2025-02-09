use entities::user::{ActiveModel as UserActiveModel, Model as UserModel};
use sea_orm::IntoActiveValue;

use super::sea_role_mapper::SeaRoleMapper;
use super::SeaMapper;
use crate::domain::domain_entities::user::User;

pub struct SeaUserMapper;

impl SeaMapper<User, UserModel, UserActiveModel> for SeaUserMapper {
    fn entity_into_model(entity: User) -> UserModel {
        UserModel {
            id: entity.id(),
            nickname: entity.nickname().to_string(),
            password: entity.password().to_string(),
            role: entity.role().map(SeaRoleMapper::into_model),
            created_at: entity.created_at(),
            last_login: entity.last_login(),
        }
    }

    fn entity_into_active_model(entity: User) -> UserActiveModel {
        UserActiveModel {
            id: entity.id().into_active_value(),
            nickname: entity.nickname().to_string().into_active_value(),
            password: entity.password().to_string().into_active_value(),
            role: sea_orm::ActiveValue::Set(entity.role().map(SeaRoleMapper::into_model)),
            created_at: entity.created_at().into_active_value(),
            last_login: entity.last_login().into_active_value(),
        }
    }

    fn active_model_into_entity(active_model: UserActiveModel) -> User {
        User::new_from_existing(
            active_model.id.unwrap(),
            active_model.nickname.unwrap(),
            active_model.password.unwrap(),
            active_model.created_at.unwrap(),
            active_model.last_login.unwrap(),
            active_model.role.unwrap().map(SeaRoleMapper::into_entity),
        )
    }

    fn model_into_entity(model: UserModel) -> User {
        User::new_from_existing(
            model.id.to_owned(),
            model.nickname.to_owned(),
            model.password.to_owned(),
            model.created_at.to_owned(),
            model.last_login.to_owned(),
            model.role.map(SeaRoleMapper::into_entity),
        )
    }
}
