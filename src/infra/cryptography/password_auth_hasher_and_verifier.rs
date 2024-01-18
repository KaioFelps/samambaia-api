use crate::domain::cryptography::{hasher::HasherTrait, comparer::ComparerTrait};
use password_auth::generate_hash;
use password_auth::verify_password;

pub struct PasswordAuthHasherAndVerifier {}

impl HasherTrait for PasswordAuthHasherAndVerifier {
    fn hash(&self, password: String) -> String {
        generate_hash(password)
    }
}

impl ComparerTrait for PasswordAuthHasherAndVerifier {
    fn compare(&self, password: &String, hashed_password: &String) -> bool {
        let result = verify_password(password, hashed_password);

        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }
}