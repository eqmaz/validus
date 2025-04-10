use app_core::prelude::*;

use crate::service::trading_service::*;

#[allow(unused)]
fn pause(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

pub fn run(app: &mut AppContext) -> Result<(), AppError> {
    sout!("App Started!");
    if app.feature_enabled("dev_mode") {
        sout!("Developer mode is ON");
    }

    trade_hello_world()?;
    //pause(100);

    trade_scenario_1()?;
    //pause(100);

    trade_scenario_2()?;
    //pause(100);

    trade_scenario_3()?;
    //pause(100);

    trade_history_view()?;
    //pause(100);

    trade_hist_diff()?;

    Ok(())
}
