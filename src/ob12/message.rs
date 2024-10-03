use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[derive(Clone, Debug)]
#[json]
pub struct OB12MessageSegData {
    pub r#type: String,
    pub data: JSONValue,
    pub alt_message: Option<String>,
}
