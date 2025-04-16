use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::Router as AxumRouter,
    Json,
};
use openapi::server; // Generated from OpenAPI spec
use serde_json::json;
use std::sync::Arc;

use crate::api::rest::impls::RestApiImpl;

async fn json_rejection_handler(req: Request<Body>, next: Next) -> Response {
    let response = next.run(req).await;

    if response.status() == StatusCode::BAD_REQUEST {
        let body = json!({
            "error": "BadRequest",
            "message": "Failed to parse JSON payload"
        });
        return (StatusCode::BAD_REQUEST, Json(body)).into_response();
    }

    response
}

pub fn create_rest_router() -> AxumRouter {
    let api_impl = Arc::new(RestApiImpl::default());
    server::new(api_impl).layer(from_fn(json_rejection_handler))
}
