use dotenvy::dotenv;
use entities::user::Column;
use hubbitos_backend::domain::services::comment_on_article_service::CommentOnArticleParams;
// use hubbitos_backend::domain::services::create_article_service::CreateArticleParams;
// use hubbitos_backend::domain::services::fetch_many_articles_service::FetchManyArticlesParams;


use hubbitos_backend::domain::factories::{comment_on_article_service_factory, create_article_service_factory, create_user_service_factory};
use hubbitos_backend::domain::services::create_user_service::CreateUserParams;
use hubbitos_backend::domain::domain_entities::role::Role;
use hubbitos_backend::infra::sea::mappers::sea_user_mapper::SeaUserMapper;
use hubbitos_backend::infra::sea::sea_service::SeaService;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::uuid;

use env_logger::{self, Target};

// use entities::article::Entity as ArticleEntity;
// use entities::article::Column as ArticleColumn;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::new().parse_env("RUST_LOG").target(Target::Stdout).init();        

    let create_user_service = create_user_service_factory::exec().await;

    let _ = create_user_service.exec_with_custom_role(CreateUserParams {
        nickname: "Floricultor".to_string(),
        password: "123456".to_owned(),
    }, Role::Ceo).await;

    // let authenticate_user_service = authenticate_user_service_factory::exec().await;

    // let tokens = authenticate_user_service.exec(AuthenticateUserParams { nickname: "Floricultor".to_string(), password: "123456".to_string() }).await;

    let floricultor_user = entities::user::Entity::find().filter(Column::Nickname.eq("Floricultor".to_owned())).one(&SeaService::new().await.db).await.unwrap().unwrap();
    let _floricultor_user = SeaUserMapper::model_to_user(floricultor_user);
    // let change_password_service = change_password_service_factory::exec().await;

    let _create_article_service = create_article_service_factory::exec().await;

    // let _ = create_article_service.exec(CreateArticleParams {
    //     author_id: floricultor_user.id(),
    //     content: "conteúdo da primeira notícia".to_string(),
    //     cover_url: "url".to_string(),
    //     title: "Primeira notícia".to_string()
    // }).await;

    // let _ = create_article_service.exec(CreateArticleParams {
    //     author_id: floricultor_user.id(),
    //     content: "conteúdo da segundaaaaa notícia".to_string(),
    //     cover_url: "url".to_string(),
    //     title: "SEGUNDA NOTICIAAA".to_string()
    // }).await;

    // let fetch_articles_service = fetch_many_articles_service_factory::exec().await;

    // let articles = fetch_articles_service.exec(FetchManyArticlesParams {
    //     page: None,
    //     per_page: None,
    //     query: None,
    //     query_by: None
    // }).await;

    // println!("{:#?}", articles.unwrap());

    let cas = comment_on_article_service_factory::exec().await;

    let res = cas.exec(CommentOnArticleParams {
        article_id: uuid!("a8df0e54-efd4-47cf-867a-7387cbd065ac"),
        author_id: uuid!("eb2521c7-3ea5-45c1-8c24-dff8e0d0cc42"),
        content: "comentando na primeira noticiaaaa".into()
    }).await;

    println!("{:#?}", res.unwrap());

    // assert_ne!(new_password, old_password);

    // let decoded_access_token = JwtService {}.decode_jwt(tokens.as_ref().unwrap().access_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));
    // let decoded_refresh_token = JwtService {}.decode_jwt(tokens.unwrap().refresh_token.token.clone(), DecodingKey::from_secret(&ENV_VARS.jwt_secret.as_ref()));

    // println!("DECODED ACCESS TOKEN: {:#?}. \nDECODED REFRESH TOKEN: {:#?}", decoded_access_token, decoded_refresh_token);
}
