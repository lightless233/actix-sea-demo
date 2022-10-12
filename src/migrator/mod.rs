use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

mod m20221012_000001_init;
mod m20221012_000002_add_password;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221012_000001_init::Migration),
            Box::new(m20221012_000002_add_password::Migration),
        ]
    }
}
