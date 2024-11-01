#[cfg(feature = "json")]
#[cfg(test)]
mod with_json {
    use onebot_types::ob11::MessageSeg;
    static MESSAGES: &str = include_str!("messages.json");

    #[test]
    fn ob11_messages() {
        let _json: Vec<MessageSeg> = serde_json::from_str(MESSAGES).unwrap();
        std::fs::write("/tmp/test", serde_json::to_string_pretty(&_json).unwrap()).unwrap();
    }
}
