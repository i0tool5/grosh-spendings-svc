use std::sync::Arc;

use serde::{
    Deserialize,
    Serialize,
};

use axum::{
    extract,
    Json,
    http,
};

use crate::application;

use super::parse_user_id;

#[derive(Deserialize, Serialize, Debug)]
pub struct SpendingEditRequest {
    #[serde(rename = "type")]
    pub spending_type : String,
    #[serde(rename = "date")]
    pub spending_date: time::Date,
    pub description: Option<String>,
    pub amount: i64,
}

pub async fn spending_edit_handler(
    state: extract::State<Arc<application::State>>,
    headers: http::HeaderMap,
    extract::Path(spending_id): extract::Path<uuid::Uuid>,
    Json(payload): Json<SpendingEditRequest>,
) -> Result<application::Response<String>, application::errors::AppError> {
    let user_id = parse_user_id(headers)?;
    let cmd = application::commands::SpendingEditCommand::new(
        spending_id,
        user_id,
        payload.spending_type,
        payload.spending_date,
        payload.description,
        payload.amount
    );

    cmd.validate()?;
    tracing::info!("edit spending command: {:?}", cmd);

    let result = state.spending_edit_command_handler.handle(cmd).await;
    
    if result.is_err() {
        tracing::error!("failed to edit spending");
        return Err(application::errors::INTERNAL_SERVER_ERROR);
    }

    Ok(application::Response{
        status_code: http::StatusCode::OK,
        data: String::from(""),
    })
}
