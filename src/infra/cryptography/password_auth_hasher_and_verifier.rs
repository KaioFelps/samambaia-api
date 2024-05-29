use crate::domain::cryptography::both::HasherAndComparerTrait;
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

impl HasherAndComparerTrait for PasswordAuthHasherAndVerifier {}

#[cfg(test)]
#[derive(Clone)]
pub struct FakeAuthHasherAndVerifier;

#[cfg(test)]
impl HasherTrait for FakeAuthHasherAndVerifier {
    fn hash(&self, password: String) -> String {
        let mut password = password;
        password.push_str("--hashed");
        password
    }
}

#[cfg(test)]
impl ComparerTrait for FakeAuthHasherAndVerifier {
    fn compare(&self, password: &String, hashed_password: &String) -> bool {
        let mut password = password.clone();
        password.push_str("--hashed");
        
        if &password == hashed_password {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
impl HasherAndComparerTrait for FakeAuthHasherAndVerifier {}