use ob_types_base::JSONValue;
use ob_types_macro::json;

#[json]
pub struct MessageEvent {
    pub sub_type: String,
    #[serde(flatten)]
    pub r#kind: MessageKind,
    #[serde(flatten)]
    pub extra: JSONValue,
}

#[json]
pub enum MessageKind {
    
}
