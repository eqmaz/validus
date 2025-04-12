use crate::api::start_rest_server_bg;
use crate::service::trading_service::*;
use app_core::prelude::*;
use std::future::Future;
use std::pin::Pin;

#[allow(unused)]
fn pause(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

pub fn run(app: &mut AppContext) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + '_>> {
    Box::pin(async move {
        out_f!("App Started!");

        // Run scenarios from brief
        if app.feature_enabled("dev_mode") {
            wout!("Dev mode enabled, running scenarios from brief");
            pause(1000);
            trade_hello_world()?;
            trade_scenario_1()?;
            trade_scenario_2()?;
            trade_scenario_3()?;
            trade_history_view()?;
            trade_hist_diff()?;
        }

        // Start the REST server in the background if enabled
        if app.feature_enabled("rest_api") {
            iout!("Starting REST server");
            start_rest_server_bg();
        }

        Ok(())
    })
}
