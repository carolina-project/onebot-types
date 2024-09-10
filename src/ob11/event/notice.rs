use std::time::Duration;

pub enum NoticeEvent {
    GroupNotice(GroupNotice),
    FriendNotice(FriendNotice),
}

pub struct GroupNotice {
    pub group_id: u64,
    pub user_id: u64,
    pub kind: GroupNoticeKind,
}

pub enum GroupNoticeKind {
    Upload(GroupUpload),
    Admin(AdminChange),
    MemberIncrease {
        sub_type: IncreaseType,
        operator_id: u64,
    },
    MemberDecrease {
        sub_type: DecreaseType,
        operator_id: u64,
    },
    Mute {
        sub_type: MuteType,
        operator_id: u64,
        duration: Duration,
    },
    Recall {
        operator_id: u64,
        message_id: u32,
    },
    /// poke target user id
    Poke(u64),
    /// lucky king user id
    LuckyKing(u64),
    Honor(GroupHonor),
}

pub struct GroupUpload {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub busid: u64,
}

pub enum AdminChange {
    Set,
    Unset,
}

pub enum IncreaseType {
    Approve,
    Invite,
}

pub enum DecreaseType {
    Leave,
    Kick,
    KickMe,
}

pub enum MuteType {
    Ban,
    LiftBan,
}

pub enum GroupHonor {
    Talkative,
    Performer,
    Emotion,
}

pub struct FriendNotice {
    pub user_id: u64,
    pub kind: FriendNoticeKind,
}

pub enum FriendNoticeKind {
    FriendAdd,
    /// recalled message's id
    Recall(u32),
}
