use ob_types_macro::onebot_action;

use crate::{ob12::{ChannelInfo, GuildInfo, UserInfo}, scalable_struct};

use super::EmptyResp;

scalable_struct! {
    #[onebot_action(GuildInfo)]
    GetGuildInfo = {
        guild_id: String,
    },
    #[onebot_action(Vec<GuildInfo>)]
    GetGuildList,
    #[onebot_action(EmptyResp)]
    SetGuildName = {
        guild_id: String,
        guild_name: String,
    },
    #[onebot_action(UserInfo)]
    GetGuildMemberInfo = {
        guild_id: String,
        user_id: String,
    },
    #[onebot_action(Vec<UserInfo>)]
    GetGuildMemberList = {
        guild_id: String,
    },
    #[onebot_action(EmptyResp)]
    LeaveGuild = {
        guild_id: String,
    },
    #[onebot_action(ChannelInfo)]
    GetChannelInfo = {
        guild_id: String,
        channel_id: String,
    },
    #[onebot_action(Vec<ChannelInfo>)]
    GetChannelList = {
        guild_id: String,
        #[serde(default)]
        joined_only: bool,
    },
    #[onebot_action(EmptyResp)]
    SetChannelName = {
        guild_id: String,
        channel_id: String,
        channel_name: String,
    },
    #[onebot_action(UserInfo)]
    GetChannelMemberInfo = {
        guild_id: String,
        channel_id: String,
        user_id: String,
    },
    #[onebot_action(Vec<UserInfo>)]
    GetChannelMemberList = {
        guild_id: String,
        channel_id: String,
    },
    #[onebot_action(EmptyResp)]
    LeaveChannel = {
        guild_id: String,
        channel_id: String,
    },
}
