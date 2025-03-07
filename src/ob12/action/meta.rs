use crate::{ob12, scalable_struct};

scalable_struct! {
    #[resp(Vec<ob12::event::RawEvent>)]
    GetLatestEvents = {
        limit: i64,
        timeout: i64
    },
    #[resp(Vec<String>)]
    #[derive(Default)]
    GetSupportedActions,
    #[resp(ob12::Status)]
    #[derive(Default)]
    GetStatus,
    #[resp(ob12::VersionInfo)]
    #[derive(Default)]
    GetVersion
}

impl GetLatestEvents {
    pub fn new() -> Self {
        Self {
            limit: 0,
            timeout: 0,
            extra: Default::default(),
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    pub fn timeout(mut self, timeout: i64) -> Self {
        self.timeout = timeout;
        self
    }
}
