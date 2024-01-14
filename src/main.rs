use env_config::EnvConfig;
use factories::create_user_service_factory;
use once_cell::sync::Lazy;
use services::create_user_service::CreateUserParams;

mod env_config;
mod services;
mod infra;
mod errors;
mod factories;

static ENV_VARS: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::from_env());

#[tokio::main]
async fn main() { 
    let create_user_service = create_user_service_factory::exec().await;

    let result = create_user_service.exec(CreateUserParams {
        nickname: "Floricultor".to_string(),
        password: "123456".to_string()
    }).await;

    println!("{:?}", dbg!(result.unwrap_err()));

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


    // let article_repository = SeaArticleRepository::new(&sea_service).await;

    // let articles = article_repository.find_all().await;

    // println!("{:#?}", articles)
}
