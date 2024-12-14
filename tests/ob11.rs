use eyre::Context;
use onebot_types::ob11::{self, RawEvent, MessageSeg};
use serde::de::DeserializeOwned;
use serde_json::Value;
static MESSAGES: &str = include_str!("ob11_messages.json");
static EVENTS: &str = include_str!("ob11_events.json");
static ACTIONS: &str = include_str!("ob11_actions.json");

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
fn ob11_messages() {
    let _msgs = parse::<MessageSeg>("message", MESSAGES);
}

#[test]
fn ob11_events() {
    let _events = parse::<RawEvent>("event", EVENTS);
}

#[test]
fn ob11_actions() {
    let _actions = parse::<ob11::action::ActionType>("action", ACTIONS);
}
