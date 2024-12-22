use crate::domain::domain_entities::article::Article;
use chrono::NaiveDateTime;

pub struct ArticlePolitics;

impl ArticlePolitics {
    pub fn article_is_recent(article: Article, now: NaiveDateTime) -> bool {
        let hours_til_no_longer_recent = 48;

        let difference_between_created_datetime_and_no =
            now.signed_duration_since(article.created_at());
        let hours_difference = difference_between_created_datetime_and_no.num_hours();

        hours_difference <= hours_til_no_longer_recent
    }
}
