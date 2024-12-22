pub mod sea_announcement_mapper;
pub mod sea_article_mapper;
pub mod sea_article_tag_mapper;
pub mod sea_comment_mapper;
pub mod sea_comment_report_mapper;
pub mod sea_comment_with_author_mapper;
pub mod sea_free_badge_mapper;
pub mod sea_role_mapper;
pub mod sea_team_role_mapper;
pub mod sea_team_user_mapper;
pub mod sea_user_mapper;

pub trait SeaMapper<Entity, Model, ActiveModel> {
    fn entity_into_model(entity: Entity) -> Model;
    fn entity_into_active_model(entity: Entity) -> ActiveModel;
    fn active_model_into_entity(active_model: ActiveModel) -> Entity;
    fn model_into_entity(model: Model) -> Entity;
}
