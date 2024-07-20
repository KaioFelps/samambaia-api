pub struct DraftArticleTag {
    value: String
}

impl DraftArticleTag {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticleTag {
    id: i32,
    value: String
}

impl ArticleTag {
    pub fn new_from_existing(id: i32, value: String) -> Self {
        Self {
            id,
            value,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn value(&self) -> &String {
        self.value()
    }

    pub fn set_value(&mut self, value: String) -> () {
        self.value = value;
    }
}