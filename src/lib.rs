#[allow(unused)]
use std::collections::HashMap;

#[cfg(feature = "ob11")]
pub mod ob11;
#[cfg(feature = "ob12")]
pub mod ob12;

#[cfg(feature = "json")]
pub(crate) fn hashmap_value_get<'de, T, D>(
    map: &mut HashMap<String, serde_json::Value>,
    key: &str,
) -> Result<T, D::Error>
where
    T: serde::de::DeserializeOwned,
    D: serde::de::Deserializer<'de>,
{
    map.remove(key)
        .ok_or_else(|| serde::de::Error::custom(format!("missing field {}", key)))
        .and_then(|r| serde_json::from_value::<T>(r).map_err(|e| serde::de::Error::custom(e)))
}

#[cfg(feature = "json")]
#[inline]
pub(crate) fn value_to_hashmap<'de, D: serde::de::Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<String, serde_json::Value>, D::Error> {
    use serde::Deserialize;
    HashMap::deserialize(deserializer).map_err(|e| serde::de::Error::custom(e))
}
