pub mod hashi_duration {
    use serde::{de::Error as _, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    // https://serde.rs/string-or-struct.html
    // https://stackoverflow.com/questions/56582722/serde-json-deserialize-any-number

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawDuration<'a> {
        String(&'a str),
        Number(u64),
    }

    //
    // Deserialize a positive integer or string with trailing 's', 'm', 'h' unit
    // as a Duration
    //
    #[allow(dead_code)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match RawDuration::deserialize(deserializer) {
            Err(_) => Ok(None), // NOTE: This handles the `null` case but also potentially masks other problems
            Ok(raw) => match raw {
                RawDuration::String(s) => {
                    let len = s.chars().count();
                    let (digits, unit) = s.split_at(len - 1);

                    let value = digits.parse::<u64>().map_err(D::Error::custom)?;
                    match unit {
                        "s" => Ok(Some(Duration::from_secs(value))),
                        "m" => Ok(Some(Duration::from_secs(value * 60))),
                        "h" => Ok(Some(Duration::from_secs(value * 60 * 60))),
                        _ => Err(D::Error::custom(format!(
                            "unknown duration unit: \"{}\"",
                            unit
                        ))),
                    }
                }
                RawDuration::Number(n) => Ok(Some(Duration::from_secs(n))),
            },
        }
    }

    //
    // Serialize a duration as "<number>s"
    //
    #[allow(dead_code)]
    pub fn serialize<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(ref duration) => {
                serializer.serialize_str(format!("{}s", duration.as_secs()).as_ref())
            }
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Debug, Default, Deserialize, Serialize)]
    #[serde(default)]
    struct HasDuration {
        #[serde(with = "hashi_duration")]
        pub duration: Option<Duration>,
    }

    #[test]
    fn deserialize_hashi_duration_integer() {
        let d1: HasDuration = serde_json::from_str(r#"{"duration":10}"#).expect("de failed");
        assert_eq!(d1.duration.unwrap(), Duration::from_secs(10));
    }

    #[test]
    fn deserialize_hashi_duration_string() {
        let d2: HasDuration = serde_json::from_str(r#"{"duration":"10s"}"#).expect("de failed");
        assert_eq!(d2.duration.unwrap(), Duration::from_secs(10));
    }

    #[test]
    fn deserialize_hashi_duration_string_units() {
        let d3: HasDuration = serde_json::from_str(r#"{"duration":"1h"}"#).expect("de failed");
        assert_eq!(d3.duration.unwrap(), Duration::from_secs(1 * 60 * 60));

        let d4: HasDuration = serde_json::from_str(r#"{"duration":"3m"}"#).expect("de failed");
        assert_eq!(d4.duration.unwrap(), Duration::from_secs(3 * 60));
    }

    #[test]
    fn deserialize_hashi_duration_empty() {
        let d: HasDuration = serde_json::from_str(r#"{}"#).expect("de failed");
        assert!(d.duration.is_none());
    }

    #[test]
    fn deserialize_hashi_duration_null() {
        let d: HasDuration = serde_json::from_str(r#"{"duration":null}"#).expect("de failed");
        assert!(d.duration.is_none());
    }
}
