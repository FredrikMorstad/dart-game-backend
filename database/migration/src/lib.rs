pub use sea_orm_migration::prelude::*;

mod m20250209_151006_create_game_table;
mod m20250209_191729_create_legs_table;
mod m20250209_201729_create_throw_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250209_151006_create_game_table::Migration),
            Box::new(m20250209_191729_create_legs_table::Migration),
            Box::new(m20250209_201729_create_throw_table::Migration),
        ]
    }
}
