use log::debug;
use sea_orm::{DbErr, sea_query, Iden};
use sea_orm_migration::{MigrationName, MigrationTrait, SchemaManager, async_trait};
use sea_orm_migration::prelude::{ColumnDef, Table};

// use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221012_000002_add_password"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        debug!("000002 up called.");
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                            .default("123456"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Name)
                    .drop_column(User::Password)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    Password,
}
