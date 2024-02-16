use dotenvy::dotenv;
use env_logger::{self, Target};

use hubbitos_backend::domain::factories::{comment_on_article_service_factory, create_comment_report_service_factory, delete_article_service_factory};
use hubbitos_backend::domain::services::comment_on_article_service::CommentOnArticleParams;
use hubbitos_backend::domain::services::delete_article_service::DeleteArticleParams;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use hubbitos_backend::infra::sea::sea_service::SeaService;
use hubbitos_backend::infra::sea::mappers::sea_user_mapper::SeaUserMapper;
use uuid::uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();

    let floricultor_user = entities::user::Entity::find()
    .filter(entities::user::Column::Nickname.eq("Floricultor".to_owned()))
    .one(&SeaService::new().await.db)
    .await
    .unwrap()
    .unwrap();

    let _floricultor_user = SeaUserMapper::model_to_user(floricultor_user);

    let _coa = comment_on_article_service_factory::exec().await;
    
    let _ccr = create_comment_report_service_factory::exec().await;

    let _das = delete_article_service_factory::exec().await;

    // let _ = _coa.exec(CommentOnArticleParams {
    //     article_id: uuid!("98afa6c5-71c3-4d44-a731-a54c6adf0c6e"),
    //     author_id: _floricultor_user.id(),
    //     content: "comentári novo vamos verr".into()
    // }).await;

    // let _res = _ccr.exec(hubbitos_backend::domain::services::create_comment_report_service::CreateCommentReportParams {
    //     user_id: _floricultor_user.id(),
    //     comment_id: uuid!("f3ced1b4-34c5-409a-9bc0-4d1823feb7c3"),
    //     content: "comentário super tóxico e ofensivo.".into()
    // }).await;


    let res = _das.exec(DeleteArticleParams {
        article_id: uuid!("98afa6c5-71c3-4d44-a731-a54c6adf0c6e"),
        user_id: _floricultor_user.id(),
    }).await;


    // println!("{:?}", res.unwrap());
    
}
