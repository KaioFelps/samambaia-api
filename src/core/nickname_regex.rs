use once_cell::sync::Lazy;
use regex::Regex;

pub static NICKNAME_REGX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9\_\-\=\?\!\@\:\;\.\,]+$").unwrap()
});