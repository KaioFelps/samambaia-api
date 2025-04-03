use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Article::Table)
                    .drop_foreign_key(Alias::new("fk-article-article-tag"))
                    .drop_column(Article::TagId)
                    .drop_column(Article::TagValue)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ArticlesTagsRel::Table)
                    .col(uuid(ArticlesTagsRel::ArticleId).not_null())
                    .col(integer(ArticlesTagsRel::TagId).not_null())
                    .primary_key(
                        Index::create()
                            .col(ArticlesTagsRel::TagId)
                            .col(ArticlesTagsRel::ArticleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-articles-tags-rel-article-id")
                            .from(ArticlesTagsRel::Table, ArticlesTagsRel::ArticleId)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-articles-tags-rel-article-tag-id")
                            .from(ArticlesTagsRel::Table, ArticlesTagsRel::TagId)
                            .to(ArticleTag::Table, ArticleTag::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ArticlesTagsRel::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Article::Table)
                    .add_column(ColumnDef::new(Article::TagId).integer())
                    .add_column(ColumnDef::new(Article::TagValue).string())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-article-article-tag")
                            .from_tbl(Article::Table)
                            .from_col(Article::TagId)
                            .to_tbl(ArticleTag::Table)
                            .to_col(ArticleTag::Id)
                            .on_delete(ForeignKeyAction::SetNull.to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ArticleTag {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    TagId,
    TagValue,
}

#[derive(DeriveIden)]
enum ArticlesTagsRel {
    Table,
    TagId,
    ArticleId,
}
