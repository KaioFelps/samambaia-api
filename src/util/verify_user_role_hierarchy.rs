use crate::domain::domain_entities::role::Role;

pub fn exec(role_1: &Role, role_2: &Role) -> bool {
    match role_1 {
        Role::User => true,
        Role::Writer => !matches!(role_2, Role::Writer | Role::User),
        Role::Editor => !matches!(role_2, Role::User | Role::Writer | Role::Editor),
        Role::Coord => matches!(role_2, Role::Ceo | Role::Principal | Role::Admin),
        Role::Admin => matches!(role_2, Role::Ceo | Role::Principal),
        Role::Principal => matches!(role_2, Role::Ceo),
        Role::Ceo => matches!(role_2, Role::Ceo),
    }
}
