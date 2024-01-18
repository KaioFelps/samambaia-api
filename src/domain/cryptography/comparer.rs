pub trait ComparerTrait {
    fn compare(&self, password: &String, hashed_password: &String) -> bool;
}