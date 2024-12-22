pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240114_032712_add_roles_to_user;
mod m20240125_212235_add_new_fields_to_articles;
mod m20240128_044449_make_article_content_not_nullable;
mod m20240128_070407_add_approved_field_to_article;
mod m20240204_210351_add_slug_to_article;
mod m20240209_010037_setup_comments_table;
mod m20240212_051303_drop_comment_article_table;
mod m20240212_051315_add_article_id_field_to_comment_table;
mod m20240213_214959_create_comment_report_table;
mod m20240214_030642_fix_comment_report_foreign_key;
mod m20240216_194500_add_is_active_to_comments;
mod m20240216_200245_remove_cascade_action_from_comment_article_foreign_key;
mod m20240216_204934_make_comment_article_id_field_nullable;
mod m20240217_210055_setup_teams_tables;
mod m20240217_212548_add_timestamp_to_team_tables;
mod m20240307_142155_change_comment_report_solved_field_to_be_solved_by;
mod m20240316_042435_drop_team_role_team_user_relation_table;
mod m20240316_042712_alter_team_user_table;
mod m20240604_054455_make_article_slug_a_unique_key;
mod m20240719_034959_create_article_tag_table_and_add_tag_to_article;
mod m20240722_224100_remove_unique_constraint_from_team_user_field;
mod m20240725_022019_create_free_badges_table;
mod m20240726_175757_rename_user_role_enum_writter_property_to_writer;
mod m20241124_033241_add_adsense_table;
mod m20241124_154522_add_extra_fields_to_announcement_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240114_032712_add_roles_to_user::Migration),
            Box::new(m20240125_212235_add_new_fields_to_articles::Migration),
            Box::new(m20240128_044449_make_article_content_not_nullable::Migration),
            Box::new(m20240128_070407_add_approved_field_to_article::Migration),
            Box::new(m20240204_210351_add_slug_to_article::Migration),
            Box::new(m20240209_010037_setup_comments_table::Migration),
            Box::new(m20240212_051303_drop_comment_article_table::Migration),
            Box::new(m20240212_051315_add_article_id_field_to_comment_table::Migration),
            Box::new(m20240213_214959_create_comment_report_table::Migration),
            Box::new(m20240214_030642_fix_comment_report_foreign_key::Migration),
            Box::new(m20240216_194500_add_is_active_to_comments::Migration),
            Box::new(
                m20240216_200245_remove_cascade_action_from_comment_article_foreign_key::Migration,
            ),
            Box::new(m20240216_204934_make_comment_article_id_field_nullable::Migration),
            Box::new(m20240217_210055_setup_teams_tables::Migration),
            Box::new(m20240217_212548_add_timestamp_to_team_tables::Migration),
            Box::new(
                m20240307_142155_change_comment_report_solved_field_to_be_solved_by::Migration,
            ),
            Box::new(m20240316_042435_drop_team_role_team_user_relation_table::Migration),
            Box::new(m20240316_042712_alter_team_user_table::Migration),
            Box::new(m20240604_054455_make_article_slug_a_unique_key::Migration),
            Box::new(m20240719_034959_create_article_tag_table_and_add_tag_to_article::Migration),
            Box::new(m20240722_224100_remove_unique_constraint_from_team_user_field::Migration),
            Box::new(m20240725_022019_create_free_badges_table::Migration),
            Box::new(m20240726_175757_rename_user_role_enum_writter_property_to_writer::Migration),
            Box::new(m20241124_033241_add_adsense_table::Migration),
            Box::new(m20241124_154522_add_extra_fields_to_announcement_table::Migration),
        ]
    }
}
