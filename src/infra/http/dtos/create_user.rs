use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::NICKNAME_REGX;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(
        // regex(path = *NICKNAME_REGX, message = "Your nickname might contain only letters, numbers, and the symbols: .,_-=?!@:;."),
        regex(path = *NICKNAME_REGX, message = "Seu nickname só pode ter letras, números e os seguintes símbolos: .,_-=?!@:;."),
        // length(min = 3, max = 22, message = "Your nickname must be between 3 and 22 chars.")
        length(min = 3, max = 22, message = "Seu nickname deve possuir entre 3 a 22 caracteres."),
        required(message = "O nickname é um campo obrigatório.")
    )]
    pub nickname: Option<String>,
    #[validate(
        length(min = 8, message = "Sua senha precisa ter no mínimo 8 caracteres."),
        required(message = "A senha é um campo obrigatório...")
    )]
    pub password: Option<String>,
}
