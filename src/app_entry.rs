use app_core::prelude::*;

use crate::service::trading_service::*;

pub fn run(app: &mut AppContext) -> Result<(), AppError> {
    sout!("App Started!");
    if app.feature_enabled("dev_mode") {
        sout!("Developer mode is ON");
    }

    // Hello world trade
    trade_hello_world()?;
    trade_scenario_1()?;
    trade_scenario_2()?;
    trade_scenario_3()?;

    Ok(())
}
