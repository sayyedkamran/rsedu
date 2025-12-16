pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251216_182846_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251216_182846_create_users_table::Migration),
        ]
    }
}
