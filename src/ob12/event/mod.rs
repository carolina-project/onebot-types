use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[derive(Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum EventType {
    Meta,
    Message,
    Notice,
    Request,
}

#[json]
pub struct Event {
    pub id: String,
    pub time: f64,
    #[serde(rename = "self")]
    pub self_: super::BotSelf,
    pub r#type: EventType,
    pub detail_type: String,
    pub sub_type: String,
    #[serde(flatten)]
    pub extra: JSONValue,
}
