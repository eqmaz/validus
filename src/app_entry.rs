use app_core::prelude::*;
use serde_json::json;

pub fn run(app: &mut AppContext) {
    sout!("App Started!");

    if app.feature_enabled("dev_mode") {
        sout!("Developer mode is ON");
    }


}
