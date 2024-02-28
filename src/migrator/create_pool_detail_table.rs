use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "create_pool_detail_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PoolDetail::Table)
                    .col(
                        ColumnDef::new(PoolDetail::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PoolDetail::TokenId).integer().not_null())
                    .col(ColumnDef::new(PoolDetail::Liquidity).string().not_null())
                    .col(ColumnDef::new(PoolDetail::Amount0).string().not_null())
                    .col(ColumnDef::new(PoolDetail::Amount1).string().not_null())
                    .col(ColumnDef::new(PoolDetail::Operator).string().not_null())
                    .col(ColumnDef::new(PoolDetail::PoolAddress).string().not_null())
                    .col(ColumnDef::new(PoolDetail::Nonce).integer().not_null())
                    .col(ColumnDef::new(PoolDetail::Token0).string().not_null())
                    .col(ColumnDef::new(PoolDetail::Token1).string().not_null())
                    .col(ColumnDef::new(PoolDetail::Fee).integer().not_null())
                    .col(ColumnDef::new(PoolDetail::TickLower).integer().not_null())
                    .col(ColumnDef::new(PoolDetail::TickUpper).integer().not_null())
                    .col(
                        ColumnDef::new(PoolDetail::FeeGrowthInside0LastX128)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PoolDetail::FeeGrowthInside1LastX128)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PoolDetail::TokensOwed0).integer().not_null())
                    .col(ColumnDef::new(PoolDetail::TokensOwed1).integer().not_null())
                    .col(
                        ColumnDef::new(PoolDetail::CreateTime)
                            .integer()
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PoolDetail::UpdateTime)
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
            .drop_table(Table::drop().table(PoolDetail::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum PoolDetail {
    Table,
    Id,
    TokenId,
    Liquidity,
    Amount0,
    Amount1,
    Nonce,
    Operator,
    Token0,
    Token1,
    PoolAddress,
    Fee,
    TickLower,
    TickUpper,
    FeeGrowthInside0LastX128,
    FeeGrowthInside1LastX128,
    TokensOwed0,
    TokensOwed1,
    CreateTime,
    UpdateTime,
}
