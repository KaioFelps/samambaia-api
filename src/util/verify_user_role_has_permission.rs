use crate::domain::domain_entities::role::Role;
use super::RolePermissions;

pub fn exec(role: &Role, expec_perm: RolePermissions) -> bool {
    let permissions_for_role = RolePermissions::get_from_role(role);

    for permission in permissions_for_role {
        if permission == expec_perm {
            return true;
        }
    }

    return false;

}