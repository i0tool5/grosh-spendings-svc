use std::sync::Arc;

use axum::{
    extract,
    http,
};

use crate::application::{self, errors::AppError};

use super::parse_user_id;

pub async fn spending_remove_handler(
    state: extract::State<Arc<application::State>>,
    headers: http::HeaderMap,
    extract::Path(spending_id): extract::Path<uuid::Uuid>,
) -> Result<application::Response<String>, application::errors::AppError> {
    let user_id = parse_user_id(headers)?;

    let cmd = application::commands::SpendingRemoveCommand::new(
       spending_id,
       user_id,
    );

    if cmd.validate().is_err() {
        return Err(AppError::new(http::StatusCode::BAD_REQUEST, ""));
    }

    let result = state.spending_remove_command_handler.handle(cmd).await;
    
    if result.is_err() {
        tracing::warn!("failed to remove spending {:?}", spending_id);
        return Err(result.err().unwrap());
    }

    Ok(application::Response{
        status_code: http::StatusCode::OK,
        data: String::from(""),
    })
}
