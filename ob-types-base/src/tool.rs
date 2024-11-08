#[cfg(feature = "json")]
pub mod duration_secs {
    use std::time::Duration;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds: u64 = serde::Deserialize::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds))
    }

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }
}

#[cfg(feature = "json")]
pub mod duration_f64 {
    use std::time::Duration;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds: f64 = serde::Deserialize::deserialize(deserializer)?;
        Ok(Duration::from_secs_f64(seconds))
    }

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(duration.as_secs_f64())
    }
}

#[cfg(feature = "json")]
pub mod duration_secs_opt {
    use std::time::Duration;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds: Option<u64> = serde::Deserialize::deserialize(deserializer)?;
        Ok(seconds.map(Duration::from_secs))
    }
    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match duration {
            Some(dur) => serializer.serialize_some(&dur.as_secs()),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(feature = "json")]
pub mod duration_str {
    use std::time::Duration;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds: &str = serde::Deserialize::deserialize(deserializer)?;
        seconds
            .parse::<u64>()
            .map(|s| Duration::from_secs(s))
            .map_err(serde::de::Error::custom)
    }

    #[inline(always)]
    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&duration.as_secs().to_string())
    }
}

#[cfg(feature = "json")]
pub mod duration_str_opt {
    use std::time::Duration;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seconds: Option<&str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(seconds
            .map(|s| s.parse::<u64>().map(Duration::from_secs))
            .transpose()
            .map_err(serde::de::Error::custom)?)
    }

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match duration {
            Some(dur) => serializer.serialize_some(&dur.as_secs().to_string()),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(feature = "json")]
pub mod from_json_str {
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, D, R>(deserializer: D) -> Result<R, D::Error>
    where
        D: serde::Deserializer<'de>,
        R: serde::de::DeserializeOwned + FromStr,
        <R as FromStr>::Err: Display,
    {
        use serde::Deserialize;
        use serde_json::Value;

        let v = serde_json::Value::deserialize(deserializer)?;
        match v {
            Value::String(s) => s.parse().map_err(serde::de::Error::custom),
            r => serde_json::from_value(r).map_err(serde::de::Error::custom),
        }
    }

    #[inline(always)]
    pub fn serialize<'de, V, S>(value: V, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        V: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}

#[cfg(feature = "json")]
pub mod str_bool {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        use serde_json::Value;

        let v = serde_json::Value::deserialize(deserializer)?;
        match v {
            Value::String(s) => match s.as_str() {
                "1" => Ok(true),
                "0" => Ok(false),
                _ => Err(serde::de::Error::custom(format!(
                    "Invalid bool string: {}",
                    s
                ))),
            },
            r => serde_json::from_value(r).map_err(serde::de::Error::custom),
        }
    }

    #[inline(always)]
    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if *value {
            serializer.serialize_str("1")
        } else {
            serializer.serialize_str("0")
        }
    }
}
