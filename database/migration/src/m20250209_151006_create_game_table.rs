use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Games::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Games::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Games::Player1).string().not_null())
                    .col(ColumnDef::new(Games::Player2).string().not_null())
                    .col(ColumnDef::new(Games::Mode).integer().not_null())
                    .col(ColumnDef::new(Games::Length).integer().not_null())
                    .col(ColumnDef::new(Games::Player1Score).integer().not_null())
                    .col(ColumnDef::new(Games::Player2Score).integer().not_null())
                    .col(ColumnDef::new(Games::Winner).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Games::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Games {
    Table,
    Id,
    Mode,
    Length,
    Player1,
    Player2,
    Player1Score,
    Player2Score,
    Winner,
}
