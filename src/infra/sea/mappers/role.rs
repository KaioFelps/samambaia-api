use entities::sea_orm_active_enums::Role as RoleModel;

use crate::domain::domain_entities::role::Role;

pub struct RoleMapper {}

impl RoleMapper {
    pub fn to_sea(role: Role) -> RoleModel {
        match role {
            Role::Ceo => RoleModel::Ceo,
            Role::Principal => RoleModel::Principal,
            Role::Admin => RoleModel::Admin,
            Role::Coord => RoleModel::Coord,
            Role::Editor => RoleModel::Editor,
            Role::Writter => RoleModel::Writter,
            Role::User => RoleModel::User,
        }
    }

    pub fn to_domain(role: RoleModel) -> Role {
        match role {
            RoleModel::Ceo => Role::Ceo,
            RoleModel::Principal => Role::Principal,
            RoleModel::Admin => Role::Admin,
            RoleModel::Coord => Role::Coord,
            RoleModel::Editor => Role::Editor,
            RoleModel::Writter => Role::Writter,
            RoleModel::User => Role::User,
        }
    }
}