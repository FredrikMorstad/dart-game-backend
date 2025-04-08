use sea_orm_migration::prelude::*;

use crate::m20250209_191729_create_legs_table::Legs;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rounds::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rounds::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rounds::Number).integer().not_null())
                    .col(ColumnDef::new(Rounds::LegId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-leg-rounds")
                            .from(Rounds::Table, Rounds::LegId)
                            .to(Legs::Table, Legs::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rounds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Rounds {
    Table,
    Id,
    LegId,
    Number,
}
