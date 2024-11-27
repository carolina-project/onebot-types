use ob_types_macro::data;

#[data]
pub struct RequestEvent {
    pub user_id: i64,
    #[serde(flatten)]
    pub kind: RequestKind,
    pub comment: String,
    pub flag: String,
}

#[data]
#[serde(tag = "request_type")]
pub enum RequestKind {
    #[serde(rename = "friend")]
    AddFriend,
    #[serde(rename = "group")]
    AddGroup(AddGroup),
}

#[data]
pub struct AddGroup {
    pub sub_type: AddGroupType,
    pub group_id: i64,
}

#[data]
#[serde(rename_all = "lowercase")]
pub enum AddGroupType {
    Add,
    Invite,
}
