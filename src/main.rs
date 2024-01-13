use env_config::EnvConfig;
use infra::sea::repositories::sea_article_repository::SeaArticleRepository;
use once_cell::sync::Lazy;
use repositories::article_repository::ArticleRepository;
mod env_config;
use crate::{infra::sea::{sea_service::SeaService, repositories::sea_user_repository::SeaUserRepository}, repositories::user_repository::{CreateUserParam, UserRepository}};

pub mod repositories;
mod infra;

static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

#[tokio::main]
async fn main() {
    let sea_service = SeaService::new().await;
    let user_repository = SeaUserRepository::new(&sea_service).await;

    // let user = user_repository.create(CreateUserParam {
    //     nickname: "Kaio".to_string(),
    //     password: "123".to_string()
    // }).await;

    // println!("{:#?}", user);

    // let mut new_article = entities::article::ActiveModel::new();
    // new_article.author_id = created_user.id;
    // new_article.content = ActiveValue::set(Some("Conteúdo do artigo aqui".to_string()));
    // new_article.likes = ActiveValue::set(Some(0));
    // new_article.title = ActiveValue::set("Título do meu primeiro artigo".to_string());

    // let created_article = new_article.save(&sea_service.db).await.unwrap();


    let article_repository = SeaArticleRepository::new(&sea_service).await;

    let articles = article_repository.find_all().await;

    println!("{:#?}", articles)
}
