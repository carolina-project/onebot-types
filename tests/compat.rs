use std::fs;

use eyre::Context as _;
use onebot_types::compat::message::IntoOB12Seg;
use onebot_types::ob11::{self, event as ob11event};
use onebot_types::ob12::event as ob12event;
use onebot_types::{compat::event::IntoOB12Event, ob11::event::EventKind as O11EventKind, ob12};
use serde::de::DeserializeOwned;
use serde_json::Value;

static OB11_MESSAGES: &str = include_str!("ob11_messages.json");
static OB11_EVENTS: &str = include_str!("ob11_events.json");
static OB11_ACTIONS: &str = include_str!("ob11_actions.json");

static OB12_MESSAGES: &str = include_str!("ob12_messages.json");
static OB12_EVENTS: &str = include_str!("ob12_events.json");
static OB12_ACTIONS: &str = include_str!("ob12_actions.json");

fn parse<D: DeserializeOwned>(name: &str, json: &str) -> Vec<D> {
    serde_json::from_str::<Vec<Value>>(json)
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(i, v)| -> Result<D, eyre::Error> {
            println!("#{}: {}", i, serde_json::to_string_pretty(&v).unwrap());
            serde_json::from_value(v).wrap_err_with(|| format!("Failed to parse {} #{}", name, i))
        })
        .collect::<eyre::Result<Vec<D>>>()
        .unwrap()
}

fn msg_default_convert(seg: ob11::MessageSeg) -> ob12::MessageSeg {
    match seg {
        ob11::MessageSeg::Text(text) => text.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Face(face) => face.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Image(image) => image.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Record(record) => record.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::Video(video) => video.into_ob12(()).unwrap().into(),
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
        ob11::MessageSeg::ForwardNode(forward_node) => forward_node.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::XML(xml) => xml.into_ob12(()).unwrap().into(),
        ob11::MessageSeg::JSON(json) => json.into_ob12(()).unwrap().into(),
        _ => panic!("Unhandled message segment")
    }
}

#[test]
fn ob11_to_12() {
    let events: Vec<ob11event::Event> = parse("ob11_event", OB11_EVENTS);
    let mut events_converted = Vec::<ob12event::EventType>::default();
    for ele in events {
        match ele.kind {
            O11EventKind::Meta(meta) => {
                let event = meta
                    .into_ob12(&ob12::VersionInfo {
                        r#impl: Default::default(),
                        version: Default::default(),
                        onebot_version: Default::default(),
                        extra: serde_value::Value::Map(Default::default()),
                    })
                    .unwrap();
                events_converted.push(event.0);
            }
            O11EventKind::Message(msg) => {
                let event = msg.into_ob12(("sadadsa".into(), |ev| Ok(msg_default_convert(ev)))).unwrap();
                events_converted.push(event);
            }
            _ => {}
        }
    }

    fs::write(
        "/tmp/test.json",
        serde_json::to_string_pretty(&events_converted).unwrap(),
    )
    .unwrap()
}
