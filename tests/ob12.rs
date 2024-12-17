use eyre::Context;
use onebot_types::{
    ob12::{
        action::ActionType,
        event::{message, notice, RawEvent},
        message::MessageSeg,
    },
    OBEventSelector,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

static MESSAGES: &str = include_str!("ob12_messages.json");
static EVENTS: &str = include_str!("ob12_events.json");
static ACTIONS: &str = include_str!("ob12_actions.json");

fn parse<D: DeserializeOwned>(name: &str, json: &str) -> Vec<D> {
    serde_json::from_str::<Vec<Value>>(json)
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            println!("#{}: {}", i, serde_json::to_string_pretty(&v).unwrap());
            serde_json::from_value(v).wrap_err_with(|| format!("Failed to parse {} #{}", name, i))
        })
        .collect::<eyre::Result<Vec<D>>>()
        .unwrap()
}

#[test]
fn ob12_actions() {
    let _actions = parse::<ActionType>("action", ACTIONS);
}

#[test]
fn ob12_messages() {
    let _messages = parse::<MessageSeg>("message", MESSAGES);
}

#[test]
fn ob12_events() {
    let _events = parse::<RawEvent>("event", EVENTS);
}

#[test]
fn ob12_event_selector() {
    #[derive(OBEventSelector)]
    enum Message {
        Private(message::Private),
        Group(notice::FriendIncrease),
    }
    let events = r#"
[
    {
        "self": {
            "platform": "qq",
            "user_id": "123234"
        },
        "time": 1632847927.599013,
        "type": "message",
        "detail_type": "private",
        "sub_type": "",
        "message_id": "6283",
        "message": [
            {
                "type": "text",
                "data": {
                    "text": "OneBot is not a bot"
                }
            },
            {
                "type": "image",
                "data": {
                    "file_id": "e30f9684-3d54-4f65-b2da-db291a477f16"
                }
            }
        ],
        "alt_message": "OneBot is not a bot[图片]",
        "user_id": "123456788"
    },
    {
        "self": {
            "platform": "qq",
            "user_id": "123234"
        },
        "time": 1632847927.599013,
        "type": "notice",
        "detail_type": "friend_increase",
        "sub_type": "",
        "user_id": "123456788"
    }
]
"#;
    let _events: Vec<Message> = serde_json::from_str(events).unwrap();
}
