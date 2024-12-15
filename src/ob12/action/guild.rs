use crate::{
    ob12::{ChannelInfo, GuildInfo, UserInfo},
    scalable_struct,
};

use super::EmptyResp;

scalable_struct! {
    #[resp(GuildInfo)]
    GetGuildInfo = {
        guild_id: String,
    },
    #[resp(Vec<GuildInfo>)]
    GetGuildList,
    #[resp(EmptyResp)]
    SetGuildName = {
        guild_id: String,
        guild_name: String,
    },
    #[resp(UserInfo)]
    GetGuildMemberInfo = {
        guild_id: String,
        user_id: String,
    },
    #[resp(Vec<UserInfo>)]
    GetGuildMemberList = {
        guild_id: String,
    },
    #[resp(EmptyResp)]
    LeaveGuild = {
        guild_id: String,
    },
    #[resp(ChannelInfo)]
    GetChannelInfo = {
        guild_id: String,
        channel_id: String,
    },
    #[resp(Vec<ChannelInfo>)]
    GetChannelList = {
        guild_id: String,
        #[serde(default)]
        joined_only: bool,
    },
    #[resp(EmptyResp)]
    SetChannelName = {
        guild_id: String,
        channel_id: String,
        channel_name: String,
    },
    #[resp(UserInfo)]
    GetChannelMemberInfo = {
        guild_id: String,
        channel_id: String,
        user_id: String,
    },
    #[resp(Vec<UserInfo>)]
    GetChannelMemberList = {
        guild_id: String,
        channel_id: String,
    },
    #[resp(EmptyResp)]
    LeaveChannel = {
        guild_id: String,
        channel_id: String,
    },
}
