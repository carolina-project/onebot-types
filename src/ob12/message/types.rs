use ob_types_macro::json;

use crate::scalable_data;

scalable_data! {
    Text = {
        text: String
    },
    Mention = {
        user_id: String
    },
    Location = {
        latitude: f64,
        longitude: f64,
        title: String,
        content: String
    },
    Reply = {
        message_id: String,
        user_id: Option<String>,
    },
    MentionAll
}

macro_rules! file_msgs {
    ($($typ:ident),* $(,)?) => {
        scalable_data! {$(
            $typ = {
                file_id: String
            },
        )*}
    };
}

file_msgs!(Image, Voice, Audio, Video, File);
