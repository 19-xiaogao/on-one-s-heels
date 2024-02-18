use sea_orm::DatabaseConnection;

pub use sea_orm_migration::prelude::*;
pub struct Migrator;

use crate::migrator::create_pool_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(create_pool_table::Migration)]
    }
}

pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db); // To investigate the schema
    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("pool").await?);
    Ok(())
}
