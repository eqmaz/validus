use crate::api::rest::router::create_rest_router;
//use axum::Router;
use app_core::config::config_string;
use tokio::net::TcpListener;

/// Starts the REST server on port 8080.
/// TODO - get port from config
pub async fn start_rest_server() {
    let host_port = config_string("rest.bind_on").unwrap_or_else(|| "0.0.0.0:8080".to_string());

    let listener = TcpListener::bind(host_port).await.expect("Failed to bind REST port");

    let router = create_rest_router();

    axum::serve(listener, router.into_make_service())
        .await
        .expect("REST server crashed");
}

/// Starts the REST server in the background.
pub fn start_rest_server_bg() {
    tokio::spawn(async {
        let _ = start_rest_server().await;
    });
}
