use ob_types_macro::onebot_action;

use crate::{ob12, scalable_data};

scalable_data! {
    #[onebot_action(Vec<ob12::event::Event>)]
    GetLatestVersion = {
        limit: i64,
        timeout: i64
    },
    GetSupportedActions,
    GetStatus,
    GetVersion
}

/// Http only
pub struct GetLatestVersion {
    pub limit: i64,
    pub timeout: i64,
}

#[onebot_action(Vec<String>)]
pub struct GetSupportedActions;

#[onebot_action(ob12::Status)]
pub struct GetStatus;

#[onebot_action(ob12::VersionInfo)]
pub struct GetVersion;
