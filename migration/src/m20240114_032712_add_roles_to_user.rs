use sea_orm::Iterable;
use sea_orm_migration::{prelude::*, sea_orm::EnumIter, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Role::Table)
                    .values(Role::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(User::Role).enumeration(Role::Table, Role::iter().skip(1)),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(Role::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(Role::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Role,
}

#[derive(Iden, EnumIter)]
pub enum Role {
    Table,
    #[iden = "User"]
    User,
    #[iden = "Writter"]
    Writter,
    #[iden = "Editor"]
    Editor,
    #[iden = "Coord"]
    Coord,
    #[iden = "Admin"]
    Admin,
    #[iden = "Principal"]
    Principal,
    #[iden = "Ceo"]
    Ceo,
}
