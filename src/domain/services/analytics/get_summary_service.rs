use crate::domain::repositories::summary_repository::SummaryRepositoryTrait;
use crate::domain::value_objects::count_summary::CountSummary;
use crate::error::SamambaiaError;

pub struct GetSummaryService<SR: SummaryRepositoryTrait> {
    pub summary_repository: SR,
}

impl<SR: SummaryRepositoryTrait> GetSummaryService<SR> {
    pub fn new(summary_repository: SR) -> Self {
        GetSummaryService { summary_repository }
    }

    pub async fn exec(&self) -> Result<CountSummary, SamambaiaError> {
        self.summary_repository
            .get_table_summary()
            .await
            .map_err(|err| {
                log::error!("Failed to fetch count summary in GetGetSummaryService: {err}");
                SamambaiaError::internal_err()
            })
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::domain::domain_entities::article::Article;
    use crate::domain::domain_entities::article_tag::DraftArticleTag;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::user::User;
    use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
    use crate::domain::value_objects::count_summary::CountSummary;
    use crate::tests::repositories::article_tag_repository::InMemoryArticleTagRepository;
    use crate::tests::repositories::summary_repository::get_summary_repository;

    #[tokio::test]
    async fn get_summary_service() {
        let (db, summary_repository) = get_summary_repository();
        let article_tags_repository = InMemoryArticleTagRepository::default();

        let tag = article_tags_repository
            .create(DraftArticleTag::new("Baz".into()))
            .await
            .unwrap();

        // region: --- Populating
        let db_mut = db.clone();
        {
            let mut db_mut = db_mut.write().unwrap();
            db_mut.article = vec![
                Article::new(
                    Uuid::new_v4(),
                    "foo".into(),
                    "foo content".into(),
                    "".into(),
                    "foo description".into(),
                    vec![tag.clone()],
                ),
                Article::new(
                    Uuid::new_v4(),
                    "bar".into(),
                    "bar content".into(),
                    "".into(),
                    "bar description".into(),
                    vec![tag.clone()],
                ),
            ];

            db_mut.user = vec![User::new("JohnDoe".into(), "".into(), Some(Role::User))];
        }

        // endregion: --- Populating

        let service = super::GetSummaryService::new(summary_repository);
        let response = service.exec().await;

        assert!(response.is_ok());
        assert_eq!(
            response.unwrap(),
            CountSummary {
                articles: 2,
                comments: 0,
                team_users: 0,
                users: 1
            }
        );
    }
}
