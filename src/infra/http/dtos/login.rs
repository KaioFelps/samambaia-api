use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(
        required(message = "Nickname é um campo obrigatório."),
        length(min = 1, message = "Nickname curtíssimo. Fanfic do after.")
    )]
    pub nickname: Option<String>,

    #[validate(
        required(message = "Senha é um campo obrigatório."),
        length(min = 1, message = "Com certeza, sua senha é maior que 1 caractere.")
    )]
    pub password: Option<String>,
}
