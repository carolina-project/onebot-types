#[allow(unused_imports)]
use std::time::Duration;

#[cfg(feature = "json")]
pub fn duration_from_seconds<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let seconds: Option<u64> = serde::Deserialize::deserialize(deserializer)?;
    Ok(seconds.map(Duration::from_secs))
}

#[cfg(feature = "json")]
pub fn duration_to_seconds<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(duration.as_secs())
}
