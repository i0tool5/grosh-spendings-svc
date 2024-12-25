use std::sync::Arc;

use uuid;

use crate::{application, infrastructure::repositories::spendings::Repository};

#[derive(Debug)]
pub struct SpendingRemoveCommand {
    spending_id: uuid::Uuid,
    user_id: uuid::Uuid,
}

impl SpendingRemoveCommand {
    pub fn new(
        spending_id: uuid::Uuid,
        user_id: uuid::Uuid,
    ) -> SpendingRemoveCommand{
        SpendingRemoveCommand{
            spending_id,
            user_id,
        }
    }
    pub fn validate(self: &Self) -> Result<(), String> {
        if self.spending_id.is_nil() {
            return Err(String::from("invalid spending id"));
        }

        if self.user_id.is_nil() {
            return Err(String::from("invalid user id"));
        }

        Ok(())
    }
}

pub struct SpendingRemoveCommandHandler {
    repository: Arc<Repository>,
}

impl SpendingRemoveCommandHandler {
    pub fn new(repository: Arc<Repository>) -> SpendingRemoveCommandHandler {
        SpendingRemoveCommandHandler{repository}
    }

    pub async fn handle(&self, cmd: SpendingRemoveCommand) -> anyhow::Result<(), application::errors::AppError> {
        tracing::info!("remove spending command {:?}", &cmd.spending_id);

        self.repository.remove(
            cmd.spending_id,
            cmd.user_id,
        ).await?;

        Ok(())
    }
}
