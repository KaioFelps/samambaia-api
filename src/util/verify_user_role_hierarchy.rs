use crate::domain::domain_entities::role::Role;

pub fn exec(role_1: &Role, role_2: &Role) -> bool {
    let is_valid = match role_1 {
        Role::User => true,
        Role::Writer => {
            match role_2 {
                Role::Writer => false,
                Role::User => false,
                _ => true
            }
        },
        Role::Editor => {
            match role_2 {
                Role::User => false,
                Role::Writer => false,
                Role::Editor => false,
                _ => true
            }
        },
        Role::Coord => {
            match role_2 {
                Role::Ceo => true,
                Role::Principal => true,
                Role::Admin => true,
                _ => false
            }
        },
        Role::Admin => {
            match role_2 {
                Role::Ceo => true,
                Role::Principal => true,
                _ => false
            }
        },
        Role::Principal => {
            match role_2 {
                Role::Ceo => true,
                _ => false
            }
        },
        Role::Ceo => {
            match role_2 {
                Role::Ceo => true,
                _ => false
            }
        }
    };

    is_valid
}