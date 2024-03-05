use sea_orm::DatabaseConnection;

pub use sea_orm_migration::prelude::*;
pub struct Migrator;

use crate::migrator::create_pool_detail_table;
use crate::migrator::create_pool_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_pool_table::Migration),
            Box::new(create_pool_detail_table::Migration),
        ]
    }
}

pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db); // To investigate the schema

    let is_exit_pool_table = schema_manager.has_table("pool").await?;
    let is_exit_pool_detail_table = schema_manager.has_table("pool_detail").await?;

    if is_exit_pool_table && is_exit_pool_detail_table {
        println!("Table 'pool' already exists, skipping initialization.");
        println!("Table 'pool_detail' already exists, skipping initialization.");
        return Ok(());
    };

    // TODO: bug: if pool is exit, pool_detail is not exit, so just run Migrator::refresh() function ,before data all remove, reset table column,
    // if afore add table? all data just remove.
    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("pool").await?);
    assert!(schema_manager.has_table("pool_detail").await?);
    Ok(())
}
