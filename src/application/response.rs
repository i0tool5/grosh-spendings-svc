//! This module contains application related response type.

use axum::{http, response, Json};
use serde::Serialize;
use serde_json::json;

pub struct Response<T> {
    pub status_code: http::StatusCode,
    pub data: T,
}

impl <T>response::IntoResponse for Response<T> 
where T: Serialize {
    fn into_response(self) -> response::Response {
        (self.status_code, Json(json!(self.data))).into_response()
    }
}
