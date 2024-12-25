mod spending_create_handler;
mod spendings_list_handler;
mod spending_remove_handler;
mod spending_edit_handler;

pub use spending_create_handler::*;
pub use spendings_list_handler::*;
pub use spending_remove_handler::*;
pub use spending_edit_handler::*;

use axum::http;

const USER_ID_HEADER_KEY: &str = "x-user-id";

pub(self) fn parse_user_id(headers: http::HeaderMap) -> anyhow::Result<uuid::Uuid> {
    let user_id_value = headers.get(USER_ID_HEADER_KEY);
    let hv = &http::HeaderValue::from_static("");
    let user_id_string = user_id_value.unwrap_or(hv).to_str()?;
    let user_id = uuid::Uuid::parse_str(user_id_string)?;

    Ok(user_id)
}
