use ob_types_macro::__data;
use serde::{
    de::{DeserializeOwned, IntoDeserializer},
    Deserialize, Serialize,
};
use serde_value::{DeserializerError, SerializerError};

use crate::ValueMap;

use super::{
    error::{ParseError, TypeMismatchError},
    trait_alias,
};

/// Trait representing an OneBot action.
pub trait OBAction: DeserializeOwned + Serialize {
    /// Static action name.
    const ACTION: Option<&'static str> = None;
    /// Response data type.
    type Resp;

    /// Get the name of the action.
    fn action_name(&self) -> &str {
        Self::ACTION.expect("Action name not set")
    }
}

/// Represents an OneBot 12 event.
pub trait OBEvent: DeserializeOwned + Serialize {
    const TYPE: &'static str;
    const DETAIL_TYPE: &'static str;
}

pub struct EventDesc {
    pub r#type: &'static str,
    pub detail_type: &'static str,
}

#[cfg(feature = "ob12")]
/// Trait for item deserializing from more than one kind of OneBot 12 events.
pub trait OBEventSelector {
    fn deserialize_event(event: crate::ob12::event::EventDetail) -> Result<Self, DeserializerError>
    where
        Self: Sized;

    fn serialize_event(&self) -> Result<crate::ob12::event::EventDetail, SerializerError>;

    fn get_selectable() -> &'static [EventDesc];
}

/// Represents an OneBot message segment.
pub trait OBMessage: DeserializeOwned + Serialize {
    const TYPE: &'static str;
}

#[__data]
pub struct RawMessageSeg {
    pub r#type: String,
    pub data: ValueMap,
}

impl RawMessageSeg {
    pub fn parse<T: OBMessage>(self) -> Result<T, ParseError> {
        if self.r#type != T::TYPE {
            return Err(TypeMismatchError::new(T::TYPE, self.r#type).into());
        }

        Ok(T::deserialize(self.data.into_deserializer())?)
    }

    pub fn try_from_msg<T: OBMessage>(msg: T) -> Result<Self, ParseError> {
        Ok(Self {
            r#type: T::TYPE.into(),
            data: Deserialize::deserialize(serde_value::to_value(msg)?)?,
        })
    }
}

#[__data(default)]
pub struct MessageChain(Vec<RawMessageSeg>);

impl From<Vec<RawMessageSeg>> for MessageChain {
    #[inline]
    fn from(value: Vec<RawMessageSeg>) -> Self {
        Self(value)
    }
}

impl<T: TryFrom<RawMessageSeg>> TryFrom<MessageChain> for Vec<T> {
    type Error = T::Error;

    fn try_from(value: MessageChain) -> Result<Self, Self::Error> {
        value
            .0
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, T::Error>>()
    }
}

impl MessageChain {
    #[inline]
    pub fn new(segs: Vec<RawMessageSeg>) -> Self {
        Self(segs)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn remove<T: OBMessage>(&mut self, idx: usize) -> Result<T, ParseError> {
        self.0.remove(idx).parse()
    }

    pub fn try_from_msg_trait<T: OBMessage>(seg: T) -> Result<Self, ParseError> {
        Ok(Self(vec![RawMessageSeg::try_from_msg(seg)?]))
    }

    pub fn try_from_trait<T: OBMessage>(segs: Vec<T>) -> Result<Self, ParseError> {
        Ok(Self(
            segs.into_iter()
                .map(|r| RawMessageSeg::try_from_msg(r))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    pub fn try_from_msg<T: TryInto<RawMessageSeg>>(seg: T) -> Result<Self, T::Error> {
        Ok(Self(vec![seg.try_into()?]))
    }

    pub fn try_from<T: TryInto<RawMessageSeg>>(segs: Vec<T>) -> Result<Self, T::Error> {
        Ok(Self(
            segs.into_iter()
                .map(|r| r.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    #[inline]
    pub fn inner(&self) -> &Vec<RawMessageSeg> {
        &self.0
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut Vec<RawMessageSeg> {
        &mut self.0
    }

    pub fn append<T: TryInto<RawMessageSeg>>(&mut self, seg: T) -> Result<(), T::Error> {
        self.0.push(seg.try_into()?);
        Ok(())
    }

    pub fn extend<T: TryInto<RawMessageSeg>>(&mut self, segs: Vec<T>) -> Result<(), T::Error> {
        self.0.extend(
            segs.into_iter()
                .map(|r| r.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(())
    }

    #[inline]
    pub fn into_inner(self) -> Vec<RawMessageSeg> {
        self.0
    }
}

trait_alias!(pub RespData(serde::Serialize, serde::de::DeserializeOwned));
