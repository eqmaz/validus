use app_core::AppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub struct HttpAppError(pub AppError);

impl IntoResponse for HttpAppError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            // app_core::AppError::NotFound(_) => StatusCode::NOT_FOUND,
            // app_core::AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            // app_core::AppError::Conversion(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({ "error": self.0.to_string() });

        (status, Json(body)).into_response()
    }
}
