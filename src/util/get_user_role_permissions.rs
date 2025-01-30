use crate::domain::domain_entities::role::Role;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum RolePermissions {
    AccessDashboard,

    UpdateUser,
    ChangeUserPassword,

    CreateArticle,
    UpdateArticle,
    ApproveArticle,
    DisapproveArticle,
    SeeUnapprovedArticle,
    ChangeArticleAuthor,
    DeleteArticle,

    InactivateComment,
    DeleteComment,

    SolveReport,
    DeleteReport,

    CreateNewTeamRole,
    UpdateTeamRole,
    DeleteTeamRole,

    CreateTeamUser,
    UpdateTeamUser,
    DeleteTeamUser,

    CreateArticleTag,
    UpdateArticleTag,
    DeleteArticleTag,

    CreateFreeBadge,
    UpdateFreeBadge,
    DeleteFreeBadge,

    CreateAnnouncement,
    UpdateAnnouncement,
    DeleteAnnouncement,
}

impl RolePermissions {
    pub fn get_from_role(role: &Role) -> Vec<RolePermissions> {
        use RolePermissions::*;

        let perms_user = vec![];
        let perms_writer = vec![
            AccessDashboard,
            //
            CreateArticle,
            //
            CreateFreeBadge,
            UpdateFreeBadge,
            DeleteFreeBadge,
        ];
        let perms_editor = [
            &perms_writer[..],
            &[UpdateArticle, ApproveArticle, SeeUnapprovedArticle],
        ]
        .concat();
        let perms_coord = [
            &perms_editor[..],
            &[DisapproveArticle, InactivateComment, SolveReport],
        ]
        .concat();
        let perms_admin = [
            &perms_coord[..],
            &[
                UpdateUser,
                //
                DeleteComment,
                //
                CreateTeamUser,
                UpdateTeamUser,
                DeleteTeamUser,
            ],
        ]
        .concat();
        let perms_principal = [
            &perms_admin[..],
            &[
                ChangeUserPassword,
                //
                DeleteArticle,
                DeleteReport,
                //
                CreateNewTeamRole,
                UpdateTeamRole,
                //
                ChangeArticleAuthor,
                CreateArticleTag,
                UpdateArticleTag,
                //
                CreateAnnouncement,
                UpdateAnnouncement,
                DeleteAnnouncement,
            ],
        ]
        .concat();
        let perms_ceo = [&perms_principal[..], &[DeleteTeamRole, DeleteArticleTag]].concat();

        match role {
            Role::User => perms_user,
            Role::Writer => perms_writer,
            Role::Editor => perms_editor,
            Role::Coord => perms_coord,
            Role::Admin => perms_admin,
            Role::Principal => perms_principal,
            Role::Ceo => perms_ceo,
        }
    }
}
