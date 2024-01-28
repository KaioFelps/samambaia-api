use crate::domain::domain_entities::role::Role;
use super::RolePermissions;
use super::get_user_role_permissions::exec as get_role_permissions;


pub fn exec(role: &Role, expec_perm: RolePermissions) -> bool {
    let permissions_for_role = get_role_permissions(role);

    for permission in permissions_for_role {
        if permission == expec_perm {
            return true;
        }
    }

    return false;

}