use std::io::Write;
use std::str::FromStr;

use dotenvy::dotenv;
use env_logger::{self, Target};
use hubbitos_backend::domain::factories::{comment_on_article_service_factory, fetch_many_comments_service_factory};
use hubbitos_backend::domain::services::fetch_many_comments_service::FetchManyCommentsParams;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use hubbitos_backend::infra::sea::sea_service::SeaService;
use hubbitos_backend::infra::sea::mappers::sea_user_mapper::SeaUserMapper;
use uuid::Uuid;

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
    // let _ = coa.exec(CommentOnArticleParams {
    //     article_id: uuid!("98afa6c5-71c3-4d44-a731-a54c6adf0c6e"),
    //     author_id: _floricultor_user.id(),
    //     content: "Noticia daora pra caralho mano".into()
    // }).await;
    // let _ = coa.exec(CommentOnArticleParams {
    //     article_id: uuid!("98afa6c5-71c3-4d44-a731-a54c6adf0c6e"),
    //     author_id: _floricultor_user.id(),
    //     content: "eu nao entendi direito o que é pra fazer na fase 2 do jogo >:( 🖕".into()
    // }).await;

    let fmc = fetch_many_comments_service_factory::exec().await;

    loop {
        let mut page = String::new();
        println!("Digite a página (número ou vazio): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut page).unwrap();
        let page: Option<u32> = {
            let num = page.trim().parse();

            if num.is_err() {
                None
            } else {
                Some(num.unwrap())
            }
        };

        let mut per_page: String = String::new();
        println!("Digite a quantidade por página (número ou vazio): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut per_page).unwrap();
        let per_page: Option<u32> = {
            let num = per_page.trim().parse();

            if num.is_err() {
                None
            } else {
                Some(num.unwrap())
            }
        };

        let mut article_id = String::new();
        println!("Digite o ID da notícia (ou deixe vazio para todos os comentários): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut article_id).unwrap();
        let article_id: Option<Uuid> = {
            let id = article_id.trim();
            let id = Uuid::from_str(&id);

            if id.is_err() {
                None
            } else {
                Some(id.unwrap())
            }
        };

        let result;

        if article_id.is_none() {
            result = fmc.exec(FetchManyCommentsParams {
                page,
                per_page,
                query: None,
                query_by: None
            }).await.unwrap();
        }
        else {
            result = fmc.exec_with_article_id(article_id.unwrap(), FetchManyCommentsParams {
                page,
                per_page,
                query: None,
                query_by: None
            }).await.unwrap();
        }

        println!("\r\n{:?}", result);

        println!("\r\n===================================\r\n")
    };
}
