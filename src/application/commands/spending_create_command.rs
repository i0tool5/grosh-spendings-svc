use std::{
    sync::Arc,
    time::SystemTime,
};

use axum::http;
use time;

use crate::{application, infrastructure::repositories::spendings::Repository};

#[derive(Debug)]
pub struct SpendingCreateCommand {
    pub user_id: uuid::Uuid,
    pub spending_types : Vec<String>,
    pub spending_date: Option<time::Date>,
    pub description: Option<String>,
    pub amount: i64,
}

impl SpendingCreateCommand {
    pub fn new(
        user_id: uuid::Uuid,
        spending_types: Vec<String>,
        spending_date: Option<time::Date>,
        description: Option<String>,
        amount: i64,
    ) -> SpendingCreateCommand{
        SpendingCreateCommand{
            user_id,
            spending_types,
            spending_date,
            description,
            amount,
        }
    }
    pub fn validate(self: &Self) -> Result<(), application::errors::AppError> {
        if self.amount <= 0 {
            return Err(application::errors::AppError::new(
                http::StatusCode::BAD_REQUEST,
                 "invalid amount"
            ));
        }
        Ok(())
    }
}

pub struct CreateSpendingCommandHandler {
    pub repository: Arc<Repository>,
}

impl CreateSpendingCommandHandler {
    pub fn new(repository: Arc<Repository>) -> CreateSpendingCommandHandler {
        CreateSpendingCommandHandler{repository}
    }

    pub async fn handle(&self, cmd: SpendingCreateCommand) -> anyhow::Result<(), anyhow::Error> {
        tracing::info!("create spending command");
        let spending_date: Option<time::Date> = if cmd.spending_date.is_some() {
            cmd.spending_date
        } else {
            let today_duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let today = time::Date::from_ordinal_date(1970, 1).unwrap() + today_duration;
            Some(today)
        };

        let result = self.repository.create(
            cmd.user_id,
            spending_date,
            cmd.spending_types,
            cmd.description,
            cmd.amount,
        ).await;

        result
    }
}
