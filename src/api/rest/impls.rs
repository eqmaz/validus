use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{Api, HelloResponse};
//use app_core::AppError;

#[derive(Default, Clone)]
pub struct RestApiImpl;

#[async_trait]
impl Api for RestApiImpl {
    async fn hello(&self, _method: Method, _host: Host, _cookies: CookieJar) -> Result<HelloResponse, String> {
        Ok(HelloResponse::Status200_ReturnsAWelcomeMessage(
            openapi::models::HelloResponse {
                message: Some("Hello from Validus 🤖".to_string()),
            },
        ))
    }
}
