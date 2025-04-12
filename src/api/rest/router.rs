use axum::routing::Router as AxumRouter;
use std::sync::Arc;

use crate::api::rest::impls::RestApiImpl;
use openapi::server; // server::new(...) provided by OpenAPI generator

pub fn create_rest_router() -> AxumRouter {
    server::new(Arc::new(RestApiImpl))
}
