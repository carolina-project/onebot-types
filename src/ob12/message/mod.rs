use ob_types_macro::json;

mod types;
pub use types::*;

#[json]
pub struct MessageSegRaw {
    pub r#type: String,
    pub data: serde_value::Value,
}

#[json]
#[serde(untagged)]
pub enum MessageChain {
    Array(Vec<MessageSeg>),
    String(String),
}

macro_rules! message_seg {
    ($($sg:ident),* $(,)?) => {
        #[json]
        #[serde(rename_all = "snake_case", tag = "type", content = "data")]
        pub enum MessageSeg {
            $($sg($sg),)*
            #[serde(untagged)]
            /// Extra message types or messages which missing fields.
            Other {
                r#type: String,
                data: serde_value::Value,
            }
        }

        $(
            impl From<$sg> for MessageSeg {
                fn from(sg: $sg) -> Self {
                    Self::$sg(sg)
                }
            }
        )*
    };
}

message_seg!(Text, Mention, MentionAll, Location, Reply, Image, Voice, Audio, Video, File);
