pub(self) use crate::ob11::event as ob11event;
use crate::ob12;
pub(self) use crate::ob12::event as ob12event;

pub(self) use crate::{DesResult, SerResult};

pub mod ob11to12 {
    use crate::compat::default_obj;

    use super::*;
    use ob12event::*;
    use ob_types_base::ext::ValueExt;

    impl MetaEvent {
        
    }
}

pub fn ob11_to_12(event: ob11event::Event) -> ob12event::Event {
    let self_ = ob12::BotSelf {
        platform: "ob11".into(),
        user_id: event.self_id.to_string(),
    };

    match event.kind {
        ob11event::EventKind::Message(_) => todo!(),
        ob11event::EventKind::Meta(_) => todo!(),
        ob11event::EventKind::Request(_) => todo!(),
        ob11event::EventKind::Notice(_) => todo!(),
    }
}
