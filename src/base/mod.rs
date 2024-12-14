pub mod error;

use ob_types_macro::__data;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::ValueMap;

pub mod ext;
pub mod tool;

mod macros;

pub trait OBAction: DeserializeOwned + serde::Serialize {
    const ACTION: Option<&'static str> = None;
    type Resp;

    fn action_name(&self) -> &str {
        Self::ACTION.expect("Action name not set")
    }
}

pub trait OB12Event<'de>: Deserialize<'de> + Serialize {
    const TYPE: &'static str;
    const DETAIL_TYPE: &'static str;
}

#[__data]
pub struct RawMessageSeg {
    pub r#type: String,
    pub data: ValueMap,
}
