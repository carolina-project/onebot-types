#[cfg(all(feature = "json", feature = "ob11"))]
mod with_json {
    use eyre::Context;
    use onebot_types::ob12::action::ActionType;
    use serde::de::DeserializeOwned;
    use serde_json::Value;

    static ACTIONS: &str = include_str!("ob12_actions.json");

    fn parse<D: DeserializeOwned>(name: &str, json: &str) -> Vec<D> {
        serde_json::from_str::<Vec<Value>>(json)
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                println!("#{}: {}", i, serde_json::to_string_pretty(&v).unwrap());
                serde_json::from_value(v)
                    .wrap_err_with(|| format!("Failed to parse {} #{}", name, i))
            })
            .collect::<eyre::Result<Vec<D>>>()
            .unwrap()
    }

    #[test]
    fn ob12_actions() {
        let _actions = parse::<ActionType>("action", ACTIONS);
        let ActionType::GetLatestEvents(_) = _actions[0] else {
            panic!("Expected ActionType");
        };
    }
}
