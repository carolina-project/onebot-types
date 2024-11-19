#[allow(unused)]
use std::collections::HashMap;

#[cfg(feature = "ob11")]
pub mod ob11;
#[cfg(feature = "ob12")]
pub mod ob12;

#[cfg(all(feature = "ob11", feature = "ob12"))]
pub mod compat;

#[allow(unused)]
pub(crate) fn hashmap_value_get<'de, T, D>(
    map: &mut HashMap<String, serde_value::Value>,
    key: &str,
) -> Result<T, D::Error>
where
    T: serde::de::DeserializeOwned,
    D: serde::de::Deserializer<'de>,
{
    map.remove(key)
        .ok_or_else(|| serde::de::Error::custom(format!("missing field {}", key)))
        .and_then(|r| T::deserialize(r).map_err(serde::de::Error::custom))
}

#[inline]
#[allow(unused)]
pub(crate) fn de_to_hashmap<'de, D: serde::de::Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<String, serde_value::Value>, D::Error> {
    use serde::Deserialize;
    HashMap::deserialize(deserializer).map_err(|e| serde::de::Error::custom(e))
}
