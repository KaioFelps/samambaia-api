use std::sync::LazyLock;

use regex::Regex;

pub static NICKNAME_REGX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\_\-\=\?\!\@\:\;\.\,]+$").unwrap());
