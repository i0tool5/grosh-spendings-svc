use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db
            .execute_unprepared(
                r#"CREATE TABLE spendings(
                    "id" UUID PRIMARY KEY,
                    "user_id" UUID NOT NULL,
                    "date" DATE,
                    "type" TEXT NOT NULL,
                    "description" TEXT,
                    "amount" NUMERIC NOT NULL
                );"#
            ).await?;

            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE spendings;").await?;
        Ok(())
    }
}

