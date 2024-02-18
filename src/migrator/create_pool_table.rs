use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "create_pool_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pool::Table)
                    .col(
                        ColumnDef::new(Pool::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pool::Token0).string().not_null())
                    .col(ColumnDef::new(Pool::Token1).string().not_null())
                    .col(ColumnDef::new(Pool::PoolAddress).text().not_null())
                    .col(ColumnDef::new(Pool::Fee).integer().not_null())
                    .col(ColumnDef::new(Pool::TickSpacing).integer().not_null())
                    .col(
                        ColumnDef::new(Pool::CreateTime)
                            .integer()
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Articles table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pool::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Pool {
    Table,
    Id,
    Token0,
    Token1,
    PoolAddress,
    Fee,
    TickSpacing,
    CreateTime,
}
