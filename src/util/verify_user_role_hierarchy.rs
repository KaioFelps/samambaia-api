use entities::sea_orm_active_enums::Role as UserRole;

pub fn exec(role_1: &UserRole, role_2: &UserRole) -> bool {
    let is_valid = match role_1 {
        UserRole::User => true,
        UserRole::Writter => {
            match role_2 {
                UserRole::Writter => false,
                UserRole::User => false,
                _ => true
            }
        },
        UserRole::Editor => {
            match role_2 {
                UserRole::User => false,
                UserRole::Writter => false,
                UserRole::Editor => false,
                _ => true
            }
        },
        UserRole::Coord => {
            match role_2 {
                UserRole::Ceo => true,
                UserRole::Principal => true,
                UserRole::Admin => true,
                _ => false
            }
        },
        UserRole::Admin => {
            match role_2 {
                UserRole::Ceo => true,
                UserRole::Principal => true,
                _ => false
            }
        },
        UserRole::Principal => {
            match role_2 {
                UserRole::Ceo => true,
                _ => false
            }
        },
        UserRole::Ceo => {
            match role_2 {
                UserRole::Ceo => true,
                _ => false
            }
        }
    };

    is_valid
}