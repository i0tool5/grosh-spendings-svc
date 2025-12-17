use std::sync::Arc;

use axum::http;
use time;

use crate::{application, domain, infrastructure::repositories::spendings::Repository};

#[derive(Debug)]
pub struct SpendingEditCommand {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub spending_types: Vec<String>,
    pub spending_date: time::Date,
    pub description: Option<String>,
    pub amount: i64,
}

impl SpendingEditCommand {
    pub fn new(
        id: uuid::Uuid,
        user_id: uuid::Uuid,
        spending_types: Vec<String>,
        spending_date: time::Date,
        description: Option<String>,
        amount: i64,
    ) -> SpendingEditCommand{
        SpendingEditCommand{
            id,
            user_id,
            spending_types,
            spending_date,
            description,
            amount,
        }
    }
    pub fn validate(self: &Self) -> Result<(), application::errors::AppError> {
        if self.id.is_nil() {
            return Err(application::errors::AppError::new(
                http::StatusCode::BAD_REQUEST,
                 "invalid spending id"
            ));
        }

        if self.user_id.is_nil() {
            return Err(application::errors::AppError::new(
                http::StatusCode::BAD_REQUEST,
                 "invalid user id"
            ));
        }

        if self.amount < 0 {
            return Err(application::errors::AppError::new(
                http::StatusCode::BAD_REQUEST,
                 "invalid amount"
            ));
        }

        Ok(())
    }
}

pub struct SpendingEditCommandHandler {
    pub repository: Arc<Repository>,
}

impl SpendingEditCommandHandler {
    pub fn new(repository: Arc<Repository>) -> SpendingEditCommandHandler {
        SpendingEditCommandHandler{repository}
    }

    pub async fn handle(&self, cmd: SpendingEditCommand) -> anyhow::Result<()> {
        return self.repository.update(
            domain::Spending{
                id: cmd.id,
                user_id: cmd.user_id,
                spending_date: Some(cmd.spending_date),
                spending_types: cmd.spending_types,
                description: cmd.description,
                amount: cmd.amount,
            },
        ).await;
    }
}
