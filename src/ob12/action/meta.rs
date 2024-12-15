use crate::{ob12, scalable_struct};

scalable_struct! {
    #[resp(Vec<ob12::event::Event>)]
    GetLatestEvents = {
        limit: i64,
        timeout: i64
    },
    #[resp(Vec<String>)]
    GetSupportedActions,
    #[resp(ob12::Status)]
    GetStatus,
    #[resp(ob12::VersionInfo)]
    GetVersion
}
