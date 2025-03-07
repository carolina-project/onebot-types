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

impl GetGuildInfo {
    pub fn new(guild_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            extra: Default::default(),
        }
    }
}

impl SetGuildName {
    pub fn new(guild_id: impl Into<String>, guild_name: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            guild_name: guild_name.into(),
            extra: Default::default(),
        }
    }
}

impl GetGuildMemberInfo {
    pub fn new(guild_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            user_id: user_id.into(),
            extra: Default::default(),
        }
    }
}

impl GetGuildMemberList {
    pub fn new(guild_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            extra: Default::default(),
        }
    }
}

impl LeaveGuild {
    pub fn new(guild_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            extra: Default::default(),
        }
    }
}

impl GetChannelInfo {
    pub fn new(guild_id: impl Into<String>, channel_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            extra: Default::default(),
        }
    }
}

impl GetChannelList {
    pub fn new(guild_id: impl Into<String>, joined_only: bool) -> Self {
        Self {
            guild_id: guild_id.into(),
            joined_only,
            extra: Default::default(),
        }
    }
}

impl SetChannelName {
    pub fn new(
        guild_id: impl Into<String>,
        channel_id: impl Into<String>,
        channel_name: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            channel_name: channel_name.into(),
            extra: Default::default(),
        }
    }
}

impl GetChannelMemberInfo {
    pub fn new(
        guild_id: impl Into<String>,
        channel_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            user_id: user_id.into(),
            extra: Default::default(),
        }
    }
}

impl GetChannelMemberList {
    pub fn new(guild_id: impl Into<String>, channel_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            extra: Default::default(),
        }
    }
}

impl LeaveChannel {
    pub fn new(guild_id: impl Into<String>, channel_id: impl Into<String>) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            extra: Default::default(),
        }
    }
}
