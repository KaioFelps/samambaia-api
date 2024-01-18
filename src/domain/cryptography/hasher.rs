pub trait HasherTrait {
    fn hash(&self, password: String) -> String;
}