use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum CustomError {
    BadRequest,
    TaskNotFound,
    InternalServerError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task Not Found"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
