pub mod error;

pub use error::OBResult;

pub mod ext;
pub mod tool;

pub trait OBRespData<'de>: serde::de::Deserialize<'de> + serde::Serialize {}

impl<'de, T: serde::de::Deserialize<'de> + serde::Serialize> OBRespData<'de> for T {}

pub trait OBAction<'a>: serde::Deserialize<'a> + serde::Serialize {
    const ACTION: Option<&'static str> = None;
    type Resp: OBRespData<'static>;

    fn action_name(&self) -> &str {
        Self::ACTION.expect("Action name not set")
    }
}
