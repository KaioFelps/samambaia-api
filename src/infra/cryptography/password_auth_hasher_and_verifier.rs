use crate::domain::cryptography::both::HasherAndComparerTrait;
use crate::domain::cryptography::{comparer::ComparerTrait, hasher::HasherTrait};
use password_auth::generate_hash;
use password_auth::verify_password;

pub struct PasswordAuthHasherAndVerifier;

impl HasherTrait for PasswordAuthHasherAndVerifier {
    fn hash(&self, password: String) -> String {
        generate_hash(password)
    }
}

impl ComparerTrait for PasswordAuthHasherAndVerifier {
    fn compare(&self, password: &str, hashed_password: &str) -> bool {
        let result = verify_password(password, hashed_password);

        result.is_ok()
    }
}

impl HasherAndComparerTrait for PasswordAuthHasherAndVerifier {}

#[cfg(test)]
#[derive(Clone)]
pub struct FakeAuthHasherAndVerifier;

#[cfg(test)]
impl HasherTrait for FakeAuthHasherAndVerifier {
    fn hash(&self, password: String) -> String {
        let mut password = password.to_string();
        password.push_str("--hashed");
        password
    }
}

#[cfg(test)]
impl ComparerTrait for FakeAuthHasherAndVerifier {
    fn compare(&self, password: &str, hashed_password: &str) -> bool {
        let mut password = password.to_string();
        password.push_str("--hashed");

        password == hashed_password
    }
}

#[cfg(test)]
impl HasherAndComparerTrait for FakeAuthHasherAndVerifier {}
