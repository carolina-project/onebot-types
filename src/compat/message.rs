use ob_types_base::JSONValue;
use ob_types_macro::json;

macro_rules! compat_struct {
    ($name:ident {
        $(
        $field:ident: $field_ty:ty,
        )*
    } $typ_name:literal) => {
        #[super::json(str)]
        pub struct $name {
            $(
            pub $field: $field_ty,
            )*
        }

        impl $name {
            pub const TYPE: &'static str = $typ_name;
        }
    };
    ($name:ident  $typ_name:literal) => {
        #[super::json(str)]
        pub struct $name;

        impl $name {
            pub const TYPE: &'static str = $typ_name;
        }
    };
    ($name:ident($en:ty)  $typ_name:literal) => {
        #[super::json(str)]
        pub struct $name(pub $en);

        impl std::ops::Deref for $name {
            type Target = $en;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            pub const TYPE: &'static str = $typ_name;
        }
    };
}

macro_rules! define_compat_types {
    ($(
        $name:ident $({
            $(
            $field:ident: $field_ty:ty
            ),*
        })? $(($en:ty))? $ob11_name:literal
    ),* $(,)?) => {
        pub mod types {
            use crate::ob11::message as ob11message;
        $(
            compat_struct!($name $({
                $(
                    $field: $field_ty,
                )*
            })? $(($en))? $ob11_name);

            impl From<$name> for super::OB12CompatSeg {
                fn from(seg: $name) -> Self {
                    Self::$name(seg)
                }
            }
        )*
        }

        /// OneBot 11 compatible segments in OneBot 12, **only used in OneBot 12 protocol**!
        #[json(serde(tag = "type"))]
        pub enum OB12CompatSeg {
            $(
            #[serde(rename = $ob11_name)]
            $name(types::$name),
            )*
        }

        #[cfg(feature = "json")]
        impl OB12CompatSeg {
            pub fn from_data(ty_name: &str, data: JSONValue) -> Option<serde_json::Result<Self>> {
                let value: serde_json::Value = data.into();
                match ty_name {
                    $(
                    types::$name::TYPE => Some(serde_json::from_value(value)),
                    )*
                    _ => None,
                }
            }

            pub fn to_data(&self) -> serde_json::Result<(&str, JSONValue)> {
                match self {
                    $(
                        OB12CompatSeg::$name(seg) => Ok(
                            (types::$name::TYPE, serde_json::to_value(seg)?.into())
                        ),
                    )*
                }
            }
        }
    };
}

define_compat_types! (
    Face(ob11message::Face) "ob11.face",
    Dice "ob11.dice",
    Rps "ob11.rps",
    Shake "ob11.shake",
    Poke(ob11message::Poke) "ob11.poke",
    Anonymous "ob11.anonymous",
    Share(ob11message::Share) "ob11.share",
    Contact(ob11message::Contact) "ob11.contact",
    Location(ob11message::Location) "ob11.location",
    Music(ob11message::Music) "ob11.music",
    Forward(ob11message::Forward) "ob11.forward",
    ForwardNode(ob11message::ForwardNode) "ob11.node",
    XML(ob11message::XML) "ob11.xml",
    JSON(ob11message::JSON) "ob11.json",
);

pub mod ob11to12 {
    use super::*;
    use ob_types_base::JSONValue;

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

    #[inline]
    fn default_obj() -> JSONValue {
        JSONValue::Object(Default::default())
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
                    extra: JSONValue::Object(Default::default()),
                }),
                ob11message::AtTarget::All => OB12Mention::MentionAll,
            }
        }
    }

    no_field_wrap!(Rps, Dice, Shake);
    single_field_wrap!(Poke);
    no_field_wrap!(Anonymous);
    single_field_wrap!(Share, Contact, Location);

    impl ob11message::Reply {
        pub fn to_ob12(self, user_id: Option<String>) -> ob12message::Reply {
            ob12message::Reply {
                message_id: self.id.to_string(),
                user_id,
                extra: default_obj(),
            }
        }
    }

    single_field_wrap!(Music, Forward, ForwardNode, XML, JSON);
}
