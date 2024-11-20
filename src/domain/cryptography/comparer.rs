pub trait ComparerTrait {
    fn compare(&self, password: &str, hashed_password: &str) -> bool;
}
