use ob_types_macro::native_data;

pub struct RequestEvent {
    pub user_id: u64,
    pub kind: RequestKind,
    pub comment: String,
    pub flag: String,
}

pub enum RequestKind {
    AddFriend,
    AddGroup(AddGroup),
}

pub struct AddGroup {
    pub sub_type: AddGroupType,
    pub group_id: u64,
}

#[native_data(serde(rename_all = "lowercase"))]
pub enum AddGroupType {
    Add,
    Invite,
}
