use std::{marker::PhantomData, time::Duration};

use ob_types_base::{OBAction, OBRespData};
use ob_types_macro::{json, onebot_action, OBRespData};

use crate::ob11::{
    event::{message::AnonymousSender, request::AddGroupType},
    message::MessageChain,
};

#[cfg(feature = "json")]
use crate::value_to_hashmap;
#[cfg(feature = "json")]
use ob_types_base::tool::duration_secs_opt;

use super::bot::MessageResp;
use super::EmptyResp;

#[onebot_action(MessageResp)]
pub struct SendGroupMsg {
    pub group_id: i64,
    pub message: MessageChain,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupKick {
    pub group_id: i64,
    pub user_id: i64,
    pub reject_add_request: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupBan {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(with = "duration_secs_opt")]
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum AnonymousFlag {
    Sender(AnonymousSender),
    Flag(String),
}

#[cfg(feature = "json")]
mod serde_impl_anon {
    use serde::Deserialize;
    use serde_json::Value;

    use super::AnonymousFlag;

    impl<'de> Deserialize<'de> for AnonymousFlag {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = Value::deserialize(deserializer)?;
            match value {
                Value::String(flag) => Ok(AnonymousFlag::Flag(flag)),
                Value::Object(obj) => {
                    let sender = serde_json::from_value(Value::Object(obj))
                        .map_err(serde::de::Error::custom)?;
                    Ok(AnonymousFlag::Sender(sender))
                }
                _ => Err(serde::de::Error::custom(
                    "AnonymousFlag must be a string or an object",
                )),
            }
        }
    }
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAnonymousBan {
    pub group_id: i64,
    #[serde(alias = "anonymous_flag", alias = "flag")]
    pub anonymous: AnonymousFlag,
    #[serde(with = "duration_secs_opt")]
    pub duration: Option<Duration>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupWholeBan {
    pub group_id: i64,
    pub enable: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAdmin {
    pub group_id: i64,
    pub user_id: i64,
    pub enable: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAnonymous {
    pub group_id: i64,
    pub enable: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupCard {
    pub group_id: i64,
    pub user_id: i64,
    pub card: Option<String>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupName {
    pub group_id: i64,
    pub group_name: String,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupLeave {
    pub group_id: i64,
    pub is_dismiss: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupSpecialTitle {
    pub group_id: i64,
    pub user_id: i64,
    pub special_title: Option<String>,
    #[cfg_attr(feature = "json", serde(with = "duration_secs_opt"))]
    pub duration: Option<Duration>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: AddGroupType,
    pub approve: Option<bool>,
    pub reason: Option<String>,
}

#[onebot_action(GroupInfo)]
pub struct GetGroupInfo {
    pub group_id: i64,
    pub no_cache: Option<bool>,
}

#[json(resp)]
pub struct GroupInfo {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: u32,
    pub max_member_count: u32,
}

#[onebot_action(Vec<GroupInfo>)]
pub struct GetGroupList;

#[onebot_action(GroupMemberInfo)]
pub struct GetGroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub no_cache: Option<bool>,
}

#[json(resp)]
pub struct GroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: String,
    pub age: u32,
    pub area: String,
    pub join_time: u32,
    pub last_sent_time: u32,
    pub level: String,
    pub role: String,
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: u32,
    pub card_changeable: bool,
}

#[onebot_action( Vec<GroupMemberInfo>)]
pub struct GetGroupMemberList {
    pub group_id: i64,
}

// get group honor
pub trait GroupHonor {
    type Output: OBRespData;
    const TYPE_NAME: &'static str;
}

/// see [get_group_honor_info](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_honor_info-%E8%8E%B7%E5%8F%96%E7%BE%A4%E8%8D%A3%E8%AA%89%E4%BF%A1%E6%81%AF)
#[json]
pub struct GetGroupHonor<Type>
where
    Type: GroupHonor,
{
    pub group_id: i64,
    _marker: PhantomData<Type>,
}

impl<T> OBAction for GetGroupHonor<T>
where
    T: GroupHonor,
{
    type Resp = T::Output;
    const ACTION: Option<&'static str> = Some("get_group_honor_info");
}

impl<Type> GetGroupHonor<Type>
where
    Type: GroupHonor,
{
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            _marker: PhantomData,
        }
    }
}

#[json]
pub struct GroupHonorUser {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

// -talkative
#[json]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: u32,
}

#[json(resp)]
pub struct GroupTalkative {
    pub group_id: i64,
    pub current_talkative: CurrentTalkative,
    pub talkative_list: Vec<GroupHonorUser>,
}

#[derive(OBRespData)]
pub struct GroupHonorList<S: GroupHonor> {
    pub group_id: i64,
    pub list: Vec<GroupHonorUser>,
    _marker: PhantomData<S>,
}

#[cfg(feature = "json")]
impl<'de, S: GroupHonor> serde::Deserialize<'de> for GroupHonorList<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value = value_to_hashmap(deserializer)?;
        let field = format!("{}_list", S::TYPE_NAME);
        Ok(Self {
            group_id: crate::hashmap_value_get::<_, D>(&mut value, "group_id")?,
            list: crate::hashmap_value_get::<_, D>(&mut value, &field)?,
            _marker: PhantomData,
        })
    }
}

#[cfg(feature = "json")]
impl<T: GroupHonor> serde::Serialize for GroupHonorList<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("group_id", &self.group_id)?;
        map.serialize_entry(&format!("{}_list", T::TYPE_NAME), &self.list)?;
        map.end()
    }
}

impl<T: GroupHonor> GroupHonorList<T> {
    pub fn new(group_id: i64, list: Vec<GroupHonorUser>) -> Self {
        Self {
            group_id,
            list,
            _marker: PhantomData,
        }
    }
}

#[json(resp)]
pub struct GroupAllHonor {
    pub group_id: i64,
    pub current_talkative: CurrentTalkative,
    pub talkative_list: Vec<GroupHonorUser>,
    pub performer_list: Vec<GroupHonorUser>,
    pub legend_list: Vec<GroupHonorUser>,
    pub strong_newbie_list: Vec<GroupHonorUser>,
    pub emotion_list: Vec<GroupHonorUser>,
}

pub mod honor {
    use super::{GroupAllHonor, GroupHonor, GroupHonorList};

    // -talkative
    pub struct Talkative;
    impl GroupHonor for Talkative {
        type Output = super::GroupTalkative;

        const TYPE_NAME: &'static str = "talkative";
    }

    // -performer
    pub struct Performer;
    impl GroupHonor for Performer {
        type Output = GroupHonorList<Self>;

        const TYPE_NAME: &'static str = "performer";
    }

    // -legend
    pub struct Legend;
    impl GroupHonor for Legend {
        type Output = GroupHonorList<Self>;

        const TYPE_NAME: &'static str = "legend";
    }

    // -strong_newbie
    pub struct StrongNewbie;
    impl GroupHonor for StrongNewbie {
        type Output = GroupHonorList<Self>;

        const TYPE_NAME: &'static str = "strong_newbie";
    }

    // -emotion
    pub struct Emotion;
    impl GroupHonor for Emotion {
        type Output = GroupHonorList<Self>;

        const TYPE_NAME: &'static str = "emotion";
    }

    // -all
    pub struct All;
    impl GroupHonor for All {
        type Output = GroupAllHonor;

        const TYPE_NAME: &'static str = "all";
    }
}
