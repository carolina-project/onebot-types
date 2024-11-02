use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[json]
pub struct OB12ActionJSONValue {
    pub action: String,
    pub echo: Option<String>,
    #[cfg_attr(feature = "json", serde(rename = "self"))]
    pub self_: super::BotSelf,
    pub params: JSONValue,
}

#[derive(Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum OB12RespStatus {
    Ok,
    Failed,
}

#[json]
pub struct OB12RespJSONValue {
    pub status: OB12RespStatus,
    pub retcode: i64,
    pub data: JSONValue,
    pub message: String,
    pub echo: Option<String>,
}
