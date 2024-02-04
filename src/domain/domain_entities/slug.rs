use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slug {
    value: String
}

impl Slug {
    // CONSTRUCTORS
    pub fn new(
        id: Uuid,
        title: String
    ) -> Self {
        let value = Self::generate_slug(id, title);
        
        Slug { value }
    }

    pub fn new_from_existing(
        value: String
    ) -> Self {
        Slug { value }
    }

    // METHODS

    fn generate_slug(id: Uuid, text: String) -> String {
        let id = id.clone().to_string();

        let id_first_hash = id.split("-").collect::<Vec<&str>>();
        let id_first_hash = id_first_hash[0].to_owned();

        let text = text.trim().to_lowercase();

        let mut normalized_text = String::new();

        for c in text.nfkd() {
            if c.is_whitespace() || c.is_ascii_whitespace() {
                normalized_text.push('-');
            }

            if c.is_alphanumeric() {
                normalized_text.push(c);
            }
        }

        let normalized_text = format!("{}-{}", id_first_hash, normalized_text);
        normalized_text
    }

    // GETTERS

    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_it_can_generate_a_slug() {
        let text = " habbo é atualizado mais uma vez por avÔ em 2024 #lacre$     ".to_string();

        let uuid = Uuid::new_v4();

        let slug = Slug::new(uuid.clone(), text);
        
        let first_hash = (uuid.to_string().split("-").collect::<Vec<&str>>())[0].to_string(); 
        
        assert_eq!(format!("{}-habbo-e-atualizado-mais-uma-vez-por-avo-em-2024-lacre", first_hash), slug.to_string());
    }

    #[test]
    fn test_if_it_can_generate_from_slug() {
        let slug = "a8f69f74-habbo-e-atualizado-mais-uma-vez-por-avo-em-2024-lacre".to_string();

        let generated_slug = Slug::new_from_existing(slug.clone());

        assert_eq!(slug, generated_slug.to_string())
    }
}