pub use sea_orm_migration::prelude::*;

mod m20250514_045755_create_table_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250514_045755_create_table_users::Migration),
        ]
    }
}
