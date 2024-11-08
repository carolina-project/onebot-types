use ob_types_macro::onebot_action;

use crate::{ob12, scalable_struct};

scalable_struct! {
    #[onebot_action(Vec<ob12::event::Event>)]
    GetLatestEvents = {
        limit: i64,
        timeout: i64
    },
    #[onebot_action(Vec<String>)]
    GetSupportedActions,
    #[onebot_action(ob12::Status)]
    GetStatus,
    #[onebot_action(ob12::VersionInfo)]
    GetVersion
}
