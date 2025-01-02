use std::{
    collections::VecDeque,
    convert::Infallible,
    ops::{Index, IndexMut},
};

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
    type Resp: DeserializeOwned;

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
}

pub trait IntoMessage {
    type Error;

    fn into_raw_msg(self) -> Result<RawMessageSeg, Self::Error>;

    fn into_msg_chain(self) -> Result<MessageChain, Self::Error>
    where
        Self: Sized,
    {
        Ok(self.into_raw_msg()?.into())
    }
}

impl IntoMessage for RawMessageSeg {
    type Error = Infallible;

    #[inline]
    fn into_raw_msg(self) -> Result<RawMessageSeg, Self::Error> {
        Ok(self)
    }
}

impl<T: OBMessage> IntoMessage for T {
    type Error = ParseError;

    fn into_raw_msg(self) -> Result<RawMessageSeg, Self::Error> {
        Ok(RawMessageSeg {
            r#type: T::TYPE.into(),
            data: Deserialize::deserialize(serde_value::to_value(self)?)?,
        })
    }
}

#[__data(default)]
pub struct MessageChain(VecDeque<RawMessageSeg>);

impl Index<usize> for MessageChain {
    type Output = RawMessageSeg;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for MessageChain {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: IntoIterator<Item = RawMessageSeg>> From<T> for MessageChain {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into_iter().collect())
    }
}

impl From<RawMessageSeg> for MessageChain {
    fn from(value: RawMessageSeg) -> Self {
        Self([value].into())
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

pub trait IntoMessageChain {
    type Error;

    fn into_msg_chain(self) -> Result<MessageChain, Self::Error>;
}

impl IntoMessageChain for MessageChain {
    type Error = Infallible;

    fn into_msg_chain(self) -> Result<MessageChain, Self::Error> {
        Ok(self)
    }
}

impl<T: IntoMessage, I: IntoIterator<Item = T>> IntoMessageChain for I {
    type Error = T::Error;

    fn into_msg_chain(self) -> Result<MessageChain, Self::Error> {
        Ok(MessageChain(
            self.into_iter()
                .map(|r| r.into_raw_msg())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl MessageChain {
    #[inline]
    pub fn new(segs: VecDeque<RawMessageSeg>) -> Self {
        Self(segs)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn remove<T: OBMessage>(&mut self, idx: usize) -> Result<T, ParseError> {
        self.0.remove(idx).ok_or(ParseError::NotFound(idx))?.parse()
    }

    #[inline]
    pub fn remove_raw(&mut self, idx: usize) -> Option<RawMessageSeg> {
        self.0.remove(idx)
    }

    #[inline]
    pub fn inner(&self) -> &VecDeque<RawMessageSeg> {
        &self.0
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut VecDeque<RawMessageSeg> {
        &mut self.0
    }

    pub fn append_back<T: IntoMessage>(&mut self, seg: T) -> Result<(), T::Error> {
        self.0.push_back(seg.into_raw_msg()?);
        Ok(())
    }

    pub fn append_front<T: IntoMessage>(&mut self, seg: T) -> Result<(), T::Error> {
        self.0.push_front(seg.into_raw_msg()?);
        Ok(())
    }

    pub fn extend<T: IntoMessage>(
        &mut self,
        segs: impl IntoIterator<Item = T>,
    ) -> Result<(), T::Error> {
        for ele in segs.into_iter() {
            self.append_back(ele)?;
        }
        Ok(())
    }

    #[inline]
    pub fn into_inner(self) -> VecDeque<RawMessageSeg> {
        self.0
    }
}

trait_alias!(pub RespData(serde::Serialize, serde::de::DeserializeOwned));
