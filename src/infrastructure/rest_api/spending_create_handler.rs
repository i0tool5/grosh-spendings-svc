use std::sync::Arc;

use serde::{
    Deserialize,
    Serialize,
};

use axum::{
    extract,
    http,
    Json,
};

use crate::application;

use super::parse_user_id;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateSpendingRequest {
    #[serde(rename = "types")]
    pub spending_types: Vec<String>,
    #[serde(rename = "date")]
    pub spending_date: Option<time::Date>,
    pub description: Option<String>,
    pub amount: i64,
}

pub async fn spending_create_handler(
    state: extract::State<Arc<application::State>>,
    headers: http::HeaderMap,
    Json(payload): Json<CreateSpendingRequest>,
) -> Result<application::Response<String>, application::errors::AppError> {
    let user_id = parse_user_id(headers)?;
    let cmd = application::commands::SpendingCreateCommand::new(
        user_id,
        payload.spending_types,
        payload.spending_date,
        payload.description,
        payload.amount
    );

    cmd.validate()?;

    let result = state.create_spending_command_handler.handle(cmd).await;
    
    if result.is_err() {
        tracing::error!("failed to create new spending");
        return Err(application::errors::INTERNAL_SERVER_ERROR);
    }

    Ok(application::Response{
        status_code: http::StatusCode::CREATED,
        data: String::from(""),
    })
}
