use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::article::{Column as ArticleColumn, Entity as ArticleEntity};
use entities::article_tag::Column as ArticleTagColumn;
use entities::user::Column as UserColumn;
use migration::extension::postgres::PgExpr;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    PaginatorTrait,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    QueryTrait,
    TransactionTrait,
};
use uuid::Uuid;

use super::sea_article_tag_repository::SeaArticleTagRepository;
use crate::core::pagination::PaginationParameters;
use crate::domain::aggregations::article_preview::{ArticlePreview, ArticlePreviewAuthor};
use crate::domain::domain_entities::article::Article;
use crate::domain::domain_entities::article_tag::ArticleTag;
use crate::domain::domain_entities::slug::Slug;
use crate::domain::repositories::article_repository::{
    ArticleQueryType,
    ArticleRepositoryTrait,
    FindManyArticlesPreviewsResponse,
    FindManyArticlesResponse,
};
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::infra::sea::mappers::sea_article_mapper::SeaArticleMapper;
use crate::infra::sea::mappers::SeaMapper;
use crate::infra::sea::sea_service::SeaService;

pub struct SeaArticleRepository<'a> {
    sea_service: &'a SeaService,
}

impl SeaArticleRepository<'_> {
    // constructor
    pub fn new(sea_service: &SeaService) -> SeaArticleRepository<'_> {
        SeaArticleRepository { sea_service }
    }
}

#[async_trait]
impl ArticleRepositoryTrait for SeaArticleRepository<'_> {
    async fn create(&self, mut article: Article) -> Result<Article, Box<dyn Error>> {
        article.flush_tags();

        let (article, tags) = SeaArticleMapper::entity_into_active_model(article);

        let article_tags = tags
            .iter()
            .map(|tag| entities::articles_tags_rel::ActiveModel {
                article_id: article.id.clone(),
                tag_id: tag.id.clone(),
            })
            .collect::<Vec<_>>();

        let created_article = article.insert(&self.sea_service.db).await?;

        let _ = entities::articles_tags_rel::Entity::insert_many(article_tags)
            .exec(&self.sea_service.db)
            .await?;

        let tags = tags
            .into_iter()
            .map(|mut tag| entities::article_tag::Model {
                id: tag.id.take().unwrap(),
                value: tag.value.take().unwrap(),
            })
            .collect();

        let created_article = SeaArticleMapper::model_into_entity((created_article, tags));

        Ok(created_article)
    }

    async fn save(&self, mut article: Article) -> Result<Article, Box<dyn Error>> {
        let transaction = self.sea_service.db.begin().await?;

        if let Some(changeset) = article.get_tags_changeset() {
            if changeset.has_changes() {
                let sea_article_tag_repository = SeaArticleTagRepository::new(&transaction);

                tokio::try_join!(
                    sea_article_tag_repository.disassociate_tags_from_article(
                        article.id(),
                        changeset.removed.iter().map(|tag| tag.id()).collect()
                    ),
                    sea_article_tag_repository.associate_tags_to_article(
                        article.id(),
                        changeset.added.iter().map(|tag| tag.id()).collect()
                    )
                )?;
            }
        }

        let (active_article, _) = SeaArticleMapper::entity_into_active_model(article.clone());

        let _ = active_article.update(&transaction).await?;
        transaction.commit().await?;

        article.flush_tags();

        Ok(article)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Box<dyn Error>> {
        let mut article = ArticleEntity::find_by_id(id)
            .find_with_related(entities::article_tag::Entity)
            .limit(1)
            .all(&self.sea_service.db)
            .await?;

        if article.is_empty() {
            return Ok(None);
        }

        let mapped_article = SeaArticleMapper::model_into_entity(article.remove(0));

        Ok(Some(mapped_article))
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, Box<dyn Error>> {
        let mut article = ArticleEntity::find()
            .filter(ArticleColumn::Slug.eq(slug.to_string()))
            .find_with_related(entities::article_tag::Entity)
            .limit(1)
            .all(&self.sea_service.db)
            .await?;

        if article.is_empty() {
            return Ok(None);
        }

        let mapped_article = SeaArticleMapper::model_into_entity(article.remove(0));

        Ok(Some(mapped_article))
    }

    async fn find_many(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesResponse, Box<dyn Error>> {
        let current_page = params.page as u64;
        let items_per_page = params.items_per_page as u64;

        let leap = (&current_page - 1) * items_per_page;

        let articles_response = ArticleEntity::find()
            .order_by_desc(ArticleColumn::CreatedAt)
            .apply_if(params.query.as_ref(), |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .apply_if(show_only_approved_state, |query_builder, approved| {
                query_builder.filter(ArticleColumn::Approved.eq(approved))
            })
            .find_with_related(entities::article_tag::Entity)
            .limit(items_per_page)
            .offset(leap)
            .all(&self.sea_service.db)
            .await?;

        let articles_count = ArticleEntity::find()
            .apply_if(params.query.as_ref(), |query_builder, query| {
                self.find_many_get_filters(query_builder, query)
            })
            .apply_if(show_only_approved_state, |query_builder, approved| {
                query_builder.filter(ArticleColumn::Approved.eq(approved))
            })
            .offset(leap)
            .count(&self.sea_service.db)
            .await?;

        let articles = articles_response
            .into_iter()
            .map(SeaArticleMapper::model_into_entity)
            .collect::<Vec<_>>();

        Ok(FindManyArticlesResponse(articles, articles_count))
    }

    async fn find_many_previews(
        &self,
        params: PaginationParameters<ArticleQueryType>,
        show_only_approved_state: Option<bool>,
    ) -> Result<FindManyArticlesPreviewsResponse, Box<dyn Error>> {
        // with limited_articles as (
        //     SELECT DISTINCT article.id
        //     FROM article
        //     LEFT JOIN article_tag_article on article_tag_article.article_id = article.id
        //     -- se o filtro for por tag
        //     WHERE article_tag_article.article_tag_id = ?
        //     -- ou se o filtro for por id do autor
        //     AND article.author_id = ?
        //     -- ou se o filtro for por titulo
        //     AND article.title = ?
        //     LIMIT ?
        //     OFFSET ?
        // )
        // SELECT
        //     article.id, article.slug, article.title, article.cover_url, article.description, article.approved, article.created_at,
        //     "user".id as user_id, "user".nickname as user_nickname,
        //     tags.id as tag_id, tags.value as tag_value
        // FROM article
        // JOIN "user" on "user".id = article.author_id
        // LEFT JOIN article_tag_article on article_tag_article.article_id = article.id
        // LEFT JOIN article_tag as tags on tags.id = article_tag_article.article_tag_id
        // WHERE article.id in (SELECT id FROM limited_articles)

        type CustomQueryResult = (
            Uuid,
            String,
            String,
            String,
            String,
            bool,
            NaiveDateTime,
            Uuid,
            String,
            i32,
            String,
        );

        let offset = ((params.page - 1) * params.items_per_page) as u64;

        let limited_articles_cte = ArticleEntity::find()
            .distinct()
            .select_only()
            .column(ArticleColumn::Id)
            .left_join(entities::articles_tags_rel::Entity)
            .apply_if(params.query.as_ref(), |query, filter| {
                self.find_many_get_filters(query, filter)
            })
            .apply_if(show_only_approved_state, |query, approved| {
                query.filter(ArticleColumn::Approved.eq(approved))
            })
            .order_by_desc(ArticleColumn::CreatedAt)
            .offset(offset)
            .limit(params.items_per_page as u64)
            .into_query();

        let (articles, articles_count): (Vec<CustomQueryResult>, u64) = tokio::try_join!(
            ArticleEntity::find()
                .select_only()
                .columns([
                    ArticleColumn::Id,
                    ArticleColumn::Slug,
                    ArticleColumn::Title,
                    ArticleColumn::CoverUrl,
                    ArticleColumn::Description,
                    ArticleColumn::Approved,
                    ArticleColumn::CreatedAt,
                ])
                .columns([UserColumn::Id, UserColumn::Nickname])
                .columns([ArticleTagColumn::Id, ArticleTagColumn::Value])
                .inner_join(entities::user::Entity)
                .left_join(entities::articles_tags_rel::Entity)
                .left_join(entities::article_tag::Entity)
                .filter(ArticleColumn::Id.in_subquery(limited_articles_cte.clone()))
                .into_tuple()
                .all(&self.sea_service.db),
            ArticleEntity::find()
                .apply_if(params.query.as_ref(), |query, filter| self
                    .find_many_get_filters(query, filter))
                .apply_if(show_only_approved_state, |query, approved| query
                    .filter(ArticleColumn::Approved.eq(approved)))
                .count(&self.sea_service.db)
        )
        .map_err(Box::new)
        .map_err(|err| err as Box<dyn Error>)?;

        let mut articles_map = HashMap::with_capacity(articles.len() / 2);

        articles.into_iter().for_each(
            |(
                id,
                slug,
                title,
                cover_url,
                description,
                approved,
                created_at,
                author_id,
                author_nickname,
                tag_id,
                tag_value,
            )| {
                let article = articles_map.entry(id).or_insert(ArticlePreview::new(
                    id,
                    cover_url,
                    title,
                    description,
                    approved,
                    Vec::new(),
                    ArticlePreviewAuthor::new(author_id, author_nickname),
                    created_at,
                    Slug::new_from_existing(slug),
                ));

                article
                    .get_tags_mut()
                    .push(ArticleTag::new_from_existing(tag_id, tag_value));
            },
        );

        let articles = articles_map
            .drain()
            .map(|(_, article)| article)
            .collect::<Vec<_>>();

        Ok(FindManyArticlesPreviewsResponse(articles, articles_count))
    }
}

impl SeaArticleRepository<'_> {
    fn find_many_get_filters(
        &self,
        query: sea_orm::Select<ArticleEntity>,
        filter: &ArticleQueryType,
    ) -> sea_orm::Select<ArticleEntity> {
        match filter {
            ArticleQueryType::Author(content) => query.filter(ArticleColumn::AuthorId.eq(*content)),
            ArticleQueryType::Title(content) => {
                // let filter = Expr::expr(Func::lower(Expr::col(ArticleColumn::Title)))
                // .ilike(format!("%{}%", content.to_lowercase()));
                query.filter(Expr::col(ArticleColumn::Title).ilike(format!("%{}%", content)))
            }
            ArticleQueryType::Tag(tag_id) => {
                query.filter(entities::articles_tags_rel::Column::TagId.eq(*tag_id))
            }
        }
    }
}
