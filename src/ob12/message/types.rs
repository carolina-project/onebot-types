use crate::scalable_struct;

scalable_struct! {
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
        scalable_struct! {$(
            #[derive(Default)]
            #[ob_types_macro::data]
            $typ = {
                file_id: String
            },
        )*}
    };
}

file_msgs!(Image, Voice, Audio, Video, File);
