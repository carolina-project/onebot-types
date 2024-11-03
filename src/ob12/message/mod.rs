use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[json]
pub struct MessageSeg {
    pub r#type: String,
    pub data: JSONValue,
    pub alt_message: Option<String>,
}
