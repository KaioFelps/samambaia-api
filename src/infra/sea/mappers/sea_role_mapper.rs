use entities::sea_orm_active_enums::Role as RoleModel;

use crate::domain::domain_entities::role::Role;

pub struct SeaRoleMapper {}

impl SeaRoleMapper {
    #[allow(clippy::wrong_self_convention)]
    pub fn into_model(role: Role) -> RoleModel {
        match role {
            Role::Ceo => RoleModel::Ceo,
            Role::Principal => RoleModel::Principal,
            Role::Admin => RoleModel::Admin,
            Role::Coord => RoleModel::Coord,
            Role::Editor => RoleModel::Editor,
            Role::Writer => RoleModel::Writer,
            Role::User => RoleModel::User,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_entity(role: RoleModel) -> Role {
        match role {
            RoleModel::Ceo => Role::Ceo,
            RoleModel::Principal => Role::Principal,
            RoleModel::Admin => Role::Admin,
            RoleModel::Coord => Role::Coord,
            RoleModel::Editor => Role::Editor,
            RoleModel::Writer => Role::Writer,
            RoleModel::User => Role::User,
        }
    }
}
