mod password_auth_hasher_and_verifier;

pub use password_auth_hasher_and_verifier::PasswordAuthHasherAndVerifier as PasswordAuthHasherAndVerifier;

#[cfg(test)]
pub use password_auth_hasher_and_verifier::FakeAuthHasherAndVerifier as MockedAuthHasherAndVerifier;