use crate::ob11::message as ob11message;

use serde::Deserialize;

macro_rules! define_compat_types {
    ($($typ:ident $name:literal),* $(,)?) => {
        pub enum OB12CompatSegment {
            $($typ(ob11message::$typ),)*
        }

        impl OB12CompatSegment {
            pub fn parse_data(
                name: &str, data: serde_value::Value
            ) -> Option<Result<Self, serde_value::DeserializerError>> {
                match name {
                    $(concat!("ob11.", $name) => {
                        Some(ob11message::$typ::deserialize(data).map(OB12CompatSegment::$typ))
                    })*
                    _ => None,
                }
            }
        }


    };
}

define_compat_types! (
    Face "face",
    Dice "dice",
    Rps "rps",
    Shake "shake",
    Poke "poke",
    Anonymous "anonymous",
    Share "share",
    Contact "contact",
    Location "location",
    Music "music",
    Forward "forward",
    ForwardNode "node",
    XML "xml",
    JSON "json",
);

pub mod ob11to12 {

    use serde_value::Value;

    use crate::ob11::message as ob11message;
    use crate::ob12::message as ob12message;

    macro_rules! no_field_wrap {
        ($($typ:ident),* $(,)?) => {
            $(
                impl ob11message::$typ {
                    pub fn to_ob12(self) -> types::$typ {
                        types::$typ {}
                    }
                }
            )*
        };
    }

    macro_rules! single_field_wrap {
        ($($typ:ident),* $(,)?) => {
            $(
                impl ob11message::$typ {
                    pub fn to_ob12(self) -> types::$typ {
                        types::$typ(self)
                    }
                }
            )*
        };
    }

    pub enum OB12Mention {
        Mention(ob12message::Mention),
        MentionAll,
    }

    #[inline(always)]
    fn default_obj() -> Value {
        Value::Map(Default::default())
    }

    impl ob11message::Text {
        pub fn into_ob12(self) -> ob12message::Text {
            ob12message::Text {
                text: self.text,
                extra: default_obj(),
            }
        }
    }

    single_field_wrap!(Face);

    impl ob11message::Image {
        pub fn to_ob12(self) -> ob12message::Image {
            ob12message::Image {
                file_id: self.file,
                extra: default_obj(),
            }
        }
    }

    impl ob11message::Record {
        pub fn to_ob12(self) -> ob12message::Voice {
            ob12message::Voice {
                file_id: self.file,
                extra: default_obj(),
            }
        }
    }

    impl ob11message::Video {
        pub fn to_ob12(self) -> ob12message::Video {
            ob12message::Video {
                file_id: self.file,
                extra: default_obj(),
            }
        }
    }

    impl ob11message::AtTarget {
        pub fn into_ob12(self) -> OB12Mention {
            match self {
                ob11message::AtTarget::QQ(id) => OB12Mention::Mention(ob12message::Mention {
                    user_id: id.to_string(),
                    extra: default_obj(),
                }),
                ob11message::AtTarget::All => OB12Mention::MentionAll,
            }
        }
    }

    no_field_wrap!(Rps, Dice, Shake);
    single_field_wrap!(Poke);
    no_field_wrap!(Anonymous);
    single_field_wrap!(Share, Contact, Location);
    single_field_wrap!(Music);

    impl ob11message::Reply {
        pub fn to_ob12(self, user_id: Option<String>) -> ob12message::Reply {
            ob12message::Reply {
                message_id: self.id.to_string(),
                user_id,
                extra: default_obj(),
            }
        }
    }

    single_field_wrap!(Forward, ForwardNode, XML, JSON);
}
