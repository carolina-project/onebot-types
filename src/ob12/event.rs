use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[derive(Clone, Copy, Debug)]
#[json(serde(rename_all = "lowercase"))]
pub enum OB12EventType {
    Meta,
    Message,
    Notice,
    Request,
}

#[derive(Clone, Debug)]
#[json]
pub struct OB12EventData {
    pub id: String,
    pub time: f64,
    #[cfg_attr(feature = "json", serde(rename = "self"))]
    pub self_: super::BotSelf,
    pub r#type: OB12EventType,
    pub detail_type: String,
    pub sub_type: String,
    pub extra: JSONValue,
}
