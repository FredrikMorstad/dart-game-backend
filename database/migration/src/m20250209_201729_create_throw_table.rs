use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250209_151006_create_game_table::Games, m20250209_191729_create_legs_table::Legs};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Throws::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Throws::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Throws::Value).string().not_null())
                    .col(ColumnDef::new(Throws::GameId).uuid().not_null())
                    .col(ColumnDef::new(Throws::LegId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-throws")
                            .from(Throws::Table, Throws::GameId)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-legs-throws")
                            .from(Throws::Table, Throws::LegId)
                            .to(Legs::Table, Legs::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Throws::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Throws {
    Table,
    Id,
    Value,
    GameId,
    LegId,
}
