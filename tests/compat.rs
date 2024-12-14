use onebot_types::compat;
use onebot_types::compat::action::bot::OB11File;
use onebot_types::compat::action::{CompatAction, IntoOB11Action, IntoOB11ActionAsync};
use onebot_types::compat::event::IntoOB12EventAsync;
use onebot_types::compat::message::{IntoOB11Seg, IntoOB11SegAsync, IntoOB12Seg, IntoOB12SegAsync};
use onebot_types::ob11::{self, action as ob11action, event as ob11event};
use onebot_types::ob12::{action as ob12action, event as ob12event};
use onebot_types::{compat::event::IntoOB12Event, ob11::event::EventKind as O11EventKind, ob12};
use serde::Deserialize;
use serde_json::Value;
use serde_value::{DeserializerError, SerializerError};

static OB11_MESSAGES: &str = include_str!("ob11_messages.json");
static OB11_EVENTS: &str = include_str!("ob11_events.json");

static OB12_MESSAGES: &str = include_str!("ob12_messages.json");
static OB12_ACTIONS: &str = include_str!("ob12_actions.json");

async fn msg_ob11_to_12(seg: ob11::MessageSeg) -> ob12::MessageSeg {
    async fn id_provider<T>(_: T) -> Result<String, SerializerError> {
        Ok("sadwawd".into())
    }
    match seg {
        ob11::MessageSeg::Text(text) => text.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Face(face) => face.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Image(image) => image.into_ob12(id_provider).await.unwrap().into(),
        ob11::MessageSeg::Record(record) => record.into_ob12(id_provider).await.unwrap().into(),
        ob11::MessageSeg::Video(video) => video.into_ob12(id_provider).await.unwrap().into(),
        ob11::MessageSeg::At(at) => at.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Rps(rps) => rps.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Dice(dice) => dice.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Shake(shake) => shake.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Poke(poke) => poke.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Anonymous(anonymous) => anonymous.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Share(share) => share.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Contact(contact) => contact.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Location(location) => location.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Music(music) => music.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Reply(reply) => reply.into_ob12(None).unwrap().into(),
        ob11::MessageSeg::Forward(forward) => forward.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Node(forward_node) => forward_node.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Xml(xml) => xml.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Json(json) => json.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Other(ob11::message::RawMessageSeg { r#type, data }) => {
            if r#type != "file" {
                panic!("Unhandled message segment: {:?}: {:?}", r#type, data)
            } else {
                ob12::MessageSeg::Other { r#type, data }
            }
        }
    }
}

async fn msg_ob12_to_11(msg: ob12::MessageSeg) -> ob11::MessageSeg {
    async fn file_provider(_: String) -> Result<String, DeserializerError> {
        Ok(String::default())
    }
    match msg {
        ob12::MessageSeg::Text(text) => text.into_ob11().unwrap().into(),
        ob12::MessageSeg::Mention(mention) => mention.into_ob11().unwrap().into(),
        ob12::MessageSeg::MentionAll(mention_all) => mention_all.into_ob11().unwrap().into(),
        ob12::MessageSeg::Location(location) => location.into_ob11().unwrap().into(),
        ob12::MessageSeg::Reply(reply) => reply.into_ob11().unwrap().into(),
        ob12::MessageSeg::Image(image) => image.into_ob11(file_provider).await.unwrap().into(),
        ob12::MessageSeg::Voice(voice) => voice.into_ob11(file_provider).await.unwrap().into(),
        ob12::MessageSeg::Audio(_) => ob11::MessageSeg::Other(ob11::message::RawMessageSeg {
            r#type: Default::default(),
            data: serde_value::Value::Map(Default::default()),
        }),
        ob12::MessageSeg::Video(video) => video.into_ob11(file_provider).await.unwrap().into(),
        ob12::MessageSeg::File(_) => ob11::MessageSeg::Other(ob11::message::RawMessageSeg {
            r#type: Default::default(),
            data: serde_value::Value::Map(Default::default()),
        }),
        ob12::MessageSeg::Other { r#type, data } => {
            compat::message::CompatSegment::parse_data(r#type, data)
                .unwrap()
                .into()
        }
    }
}

#[tokio::test]
async fn messages_ob12_to_11() {
    let messages: Vec<Value> = serde_json::from_str(OB12_MESSAGES).unwrap();

    let mut messages_converted = Vec::<ob11::message::MessageSeg>::default();
    for (i, ele) in messages.into_iter().enumerate() {
        println!("#{}: {}", i, serde_json::to_string_pretty(&ele).unwrap());
        let msg = ob12::message::MessageSeg::deserialize(ele).unwrap();
        let converted = msg_ob12_to_11(msg).await;
        messages_converted.push(converted);
    }
}

#[tokio::test]
async fn messages_ob11_to_12() {
    let messages: Vec<Value> = serde_json::from_str(OB11_MESSAGES).unwrap();

    let mut messages_converted = Vec::<ob12::message::MessageSeg>::default();
    for (i, ele) in messages.into_iter().enumerate() {
        println!("#{}: {}", i, serde_json::to_string_pretty(&ele).unwrap());
        let msg = ob11::message::MessageSeg::deserialize(ele).unwrap();
        messages_converted.push(msg_ob11_to_12(msg).await);
    }
}

#[tokio::test]
async fn events_ob11_to_12() {
    let events: Vec<Value> = serde_json::from_str(OB11_EVENTS).unwrap();

    let mut events_converted = Vec::<ob12event::EventType>::default();
    for (i, ele) in events.into_iter().enumerate() {
        println!("#{}: {}", i, serde_json::to_string_pretty(&ele).unwrap());
        let event = ob11event::RawEvent::deserialize(ele).unwrap();

        match event.detail {
            O11EventKind::Meta(meta) => {
                let event = meta
                    .into_ob12(&ob12::VersionInfo {
                        r#impl: Default::default(),
                        version: Default::default(),
                        onebot_version: Default::default(),
                        extra: Default::default(),
                    })
                    .unwrap();
                println!("{:?}", event);
                events_converted.push(event.0);
            }
            O11EventKind::Message(msg) => {
                let event = msg
                    .into_ob12(("sadadsa".into(), |ev| async {
                        Ok(msg_ob11_to_12(ev).await)
                    }))
                    .await
                    .unwrap();
                events_converted.push(event);
            }
            O11EventKind::Notice(notice) => {
                let event = notice
                    .into_ob12(("asdaw".to_string(), |_| "asdawd".to_string()))
                    .unwrap();
                events_converted.push(event);
            }
            O11EventKind::Request(request) => {
                let event = request.into_ob12("sadwa".to_string()).unwrap();
                events_converted.push(event);
            }
        }
    }
}

async fn convert_ob12_action(action: ob12action::ActionType) -> Option<ob11action::ActionType> {
    match action {
        ob12action::ActionType::GetLatestEvents(_) => None,
        ob12action::ActionType::GetSupportedActions(_) => None,
        ob12action::ActionType::GetStatus(status) => Some(status.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetVersion(version) => Some(version.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetSelfInfo(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetUserInfo(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetFriendList(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::SendMessage(action) => Some(
            action
                .into_ob11(|msg| async { Ok::<_, DeserializerError>(msg_ob12_to_11(msg).await) })
                .await
                .unwrap()
                .into(),
        ),
        ob12action::ActionType::DeleteMessage(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetGroupInfo(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetGroupList(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetGroupMemberInfo(action) => {
            Some(action.into_ob11(()).unwrap().into())
        }
        ob12action::ActionType::GetGroupMemberList(action) => {
            Some(action.into_ob11(()).unwrap().into())
        }
        ob12action::ActionType::SetGroupName(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::LeaveGroup(action) => Some(action.into_ob11(()).unwrap().into()),
        ob12action::ActionType::GetFile(action) => Some(
            action
                .into_ob11(|_| async { Some(OB11File::Record("sadwad".into())) })
                .await
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        ob12action::ActionType::Other(action) => {
            CompatAction::from_data(&action.action, action.params).unwrap();
            None
        }
        _ => None,
    }
}

#[tokio::test]
async fn ob12_actions_to_11() {
    let actions: Vec<Value> = serde_json::from_str(OB12_ACTIONS).unwrap();

    let mut actions_converted = Vec::<ob11action::ActionType>::default();

    for (i, ele) in actions.into_iter().enumerate() {
        println!("#{}: {}", i, serde_json::to_string_pretty(&ele).unwrap());
        let action = ob12action::ActionType::deserialize(ele).unwrap();
        if let Some(action) = convert_ob12_action(action).await {
            println!(
                "converted: {}",
                serde_json::to_string_pretty(&action).unwrap()
            );
            actions_converted.push(action);
        }
    }
}
