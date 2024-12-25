
pub mod spendings {

    use std::sync::Arc;

    use anyhow;
    use sea_orm::{
        prelude::Decimal as seaDecimal, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter
    };

    use crate::{application, domain, infrastructure::database::entities};

    pub struct Repository {
        db: Arc<sea_orm::DatabaseConnection>,
    }

    impl Repository {
        pub fn new(db: Arc<sea_orm::DatabaseConnection>) -> Repository {
            Repository{db}
        }

        pub async fn create(
            self: &Self,
            user_id: uuid::Uuid,
            spending_date: Option<time::Date>,
            spending_type: String,
            description: Option<String>,
            amount: i64,
        ) -> anyhow::Result<()> {
            let id = uuid::Uuid::new_v4();
            let new_spending = entities::spendings::ActiveModel{
                id: sea_orm::ActiveValue::Set(id),
                user_id: sea_orm::ActiveValue::Set(user_id),
                spending_date: sea_orm::ActiveValue::Set(spending_date),
                spending_type: sea_orm::ActiveValue::Set(spending_type),
                description: sea_orm::ActiveValue::Set(description),
                amount: sea_orm::ActiveValue::Set(seaDecimal::new(amount, 0)),
            };
    
            entities::prelude::Spendings::insert(new_spending)
            .exec(self.db.as_ref())
            .await?;
    
            Ok(())
        }

        pub async fn update(
            self: &Self,
            spending: domain::Spending,
        ) -> anyhow::Result<()> {
            
            let spending_model = entities::spendings::ActiveModel{
                id: sea_orm::ActiveValue::Set(spending.id),
                user_id: sea_orm::ActiveValue::Unchanged(spending.user_id),
                spending_date: sea_orm::ActiveValue::Set(spending.spending_date),
                spending_type: sea_orm::ActiveValue::Set(spending.spending_type),
                description: sea_orm::ActiveValue::Set(spending.description),
                amount: sea_orm::ActiveValue::Set(seaDecimal::new(spending.amount, 0)),
            };

            spending_model.update(self.db.as_ref()).await?;
    
            Ok(())
        }

        pub async fn remove(
            self: &Self,
            id: uuid::Uuid,
            user_id: uuid::Uuid,
        ) -> Result<(), application::errors::AppError> {
            let spending = entities::spendings::ActiveModel{
                id: sea_orm::ActiveValue::Set(id),
                ..Default::default()
            };
            let result = entities::prelude::Spendings::delete(spending)
            .filter(entities::spendings::Column::UserId.eq(user_id))
            .exec(self.db.as_ref())
            .await;

            match result {
                Ok(db_result) => {
                    if db_result.rows_affected == 0 {
                        return Err(application::errors::SPENDING_NOT_FOUND_ERROR);
                    }
                },
                Err(_) => {
                    return Err(application::errors::INTERNAL_SERVER_ERROR);
                },
            }
    
            Ok(())
        }
    }

}
