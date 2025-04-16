// TODO - not in use yet, was just an idea

use axum::body::Body;
use axum::{
    async_trait,
    extract::{FromRequest, Json},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::ops::Deref;

/// Wraps validated JSON data and gives nice error messages
pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S, T> FromRequest<S, Body> for ValidatedJson<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let result = Json::<T>::from_request(req, state).await;

        match result {
            Ok(Json(value)) => Ok(ValidatedJson(value)),
            Err(err) => {
                let msg = match &err {
                    axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                        "Missing or incorrect Content-Type header (expected application/json)".into()
                    }
                    axum::extract::rejection::JsonRejection::JsonDataError(e) => {
                        format!("Data error: {}", e.body_text())
                    }
                    axum::extract::rejection::JsonRejection::JsonSyntaxError(e) => {
                        format!("Syntax error: {}", e.body_text())
                    }
                    _ => format!("Invalid JSON input: {}", err),
                };

                let body = json!({
                    "error": "BadRequest",
                    "message": msg,
                });

                Err((StatusCode::BAD_REQUEST, Json(body)).into_response())
            }
        }
    }
}
