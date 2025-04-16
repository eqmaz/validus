use crate::api::{start_grpc_server_bg, start_rest_server_bg};
use crate::service::trading_service::*;
use app_core::prelude::*;
use std::future::Future;
use std::pin::Pin;

/// Helper function to run the app in a boxed future, Required for async context
pub fn run_boxed<'a>(app: &'a mut AppContext) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>> {
    Box::pin(run(app))
}

pub async fn run(app: &mut AppContext) -> Result<(), AppError> {
    out_f!("App Started!");

    if app.feature_enabled("dev_mode") {
        wout!("Dev mode enabled, running scenarios from brief");
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        trade_hello_world()?;
        trade_scenario_1()?;
        trade_scenario_2()?;
        trade_scenario_3()?;
        trade_history_view()?;
        trade_hist_diff()?;
    }

    if app.feature_enabled("rest_api") {
        iout!("Starting REST server");
        start_rest_server_bg();
    }

    if app.feature_enabled("grpc_api") {
        iout!("Starting gRPC server");
        start_grpc_server_bg();
    }

    // keep the app alive
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
    iout!("Shutdown requested");

    Ok(())
}
