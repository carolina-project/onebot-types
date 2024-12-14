use ob_types_macro::data;

use crate::ValueMap;

#[data]
pub struct RequestEvent {
    pub user_id: i64,
    #[serde(flatten)]
    pub kind: RequestKind,
    pub comment: String,
    pub flag: String,
}

#[data]
pub struct RequestRaw {
    pub request_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

#[data]
#[serde(tag = "request_type", rename_all = "snake_case")]
pub enum RequestKind {
    Friend,
    Group(AddGroup),
    Other(RequestRaw),
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
