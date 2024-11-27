use eyre::Context;
use onebot_types::ob12::{action::ActionType, event::Event, message::MessageSeg};
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
    let _events = parse::<Event>("event", EVENTS);
}
