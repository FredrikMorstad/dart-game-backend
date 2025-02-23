use sea_orm_migration::prelude::*;

use crate::m20250209_151006_create_game_table::Games;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sets::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sets::Number).integer().not_null())
                    .col(ColumnDef::new(Sets::Player1Points).integer().not_null())
                    .col(ColumnDef::new(Sets::Player2Points).integer().not_null())
                    .col(ColumnDef::new(Sets::GameId).uuid().not_null())
                    .col(ColumnDef::new(Sets::Opening).string().not_null())
                    .col(ColumnDef::new(Sets::Length).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-sets")
                            .from(Sets::Table, Sets::GameId)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sets {
    Table,
    Id,
    Number,
    Player1Points,
    Player2Points,
    Opening,
    Length,
    GameId,
}
