use crate::scalable_struct;

scalable_struct! {
    #[msg]
    Text = {
        text: String
    },
    #[msg]
    Mention = {
        user_id: String
    },
    #[msg]
    Location = {
        latitude: f64,
        longitude: f64,
        title: String,
        content: String
    },
    #[msg]
    Reply = {
        message_id: String,
        user_id: Option<String>,
    },
    #[msg]
    MentionAll
}

macro_rules! file_msgs {
    ($($typ:ident),* $(,)?) => {
        scalable_struct! {$(
            $typ = {
                file_id: String
            },
        )*}
    };
}

file_msgs!(Image, Voice, Audio, Video, File);

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Text {
            text: text.into(),
            extra: Default::default(),
        }
    }
}
