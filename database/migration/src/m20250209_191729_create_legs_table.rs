use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Legs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Legs::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Legs::Number).integer().not_null())
                    .col(ColumnDef::new(Legs::Player1Score).integer().not_null())
                    .col(ColumnDef::new(Legs::Player2Score).integer().not_null())
                    .col(ColumnDef::new(Legs::Set).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Legs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Legs {
    Table,
    Id,
    Number,
    Set,
    Player1Score,
    Player2Score,
}
