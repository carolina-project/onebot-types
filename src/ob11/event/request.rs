use ob_types_macro::json;

#[json]
pub struct RequestEvent {
    pub user_id: u64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: RequestKind,
    pub comment: String,
    pub flag: String,
}

#[json(serde(untagged))]
pub enum RequestKind {
    AddFriend,
    AddGroup(AddGroup),
}

#[json]
pub struct AddGroup {
    pub sub_type: AddGroupType,
    pub group_id: u64,
}

#[json(serde(rename_all = "lowercase"))]
pub enum AddGroupType {
    Add,
    Invite,
}
