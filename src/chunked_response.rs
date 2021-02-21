use serde::de::DeserializeOwned;
use serde_json;

pub struct Assembler
{
    buffered: Option<String>,
}

impl Assembler
{
    pub fn new() -> Self {
        Self {
            buffered: None,
        }
    }

    pub fn add<T>(&mut self, chunk: &str) -> Option<T>
    where
        T: DeserializeOwned
    {
        if self.buffered.is_none() {
            match serde_json::from_str::<T>(chunk) {
                Ok(value) => return Some(value),
                Err(_) => {
                    self.buffered = Some(String::from(chunk));
                    // TODO: provide some means of accessing the error because
                    // we likely need to differentiate between EOF parse errors
                    // and actual complete parses with syntactic errors.
                    return None
                },
            }
        };

        // Optimistic deserialize failed or have an existing buffer
        if let Some(ref mut buffer) = self.buffered {
            buffer.push_str(chunk);
            if let Ok(value) = serde_json::from_str::<T>(&buffer) {
                self.buffered = None;
                return Some(value);
            };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    type IntMap = HashMap<String, i32>;

    #[test]
    fn complete_object() {
        let chunk1 = r#"{ "one": 1 }"#;

        let mut assembler = Assembler::new();
        let value = assembler.add::<IntMap>(&chunk1).expect("deserialized value");
        assert_eq!(value.get("one"), Some(&1i32));
    }

    #[test]
    fn partial_object() {
        let chunk1 = r#"{ "one": 1"#;
        let chunk2 = r#"}"#;
        let chunk3 = r#"{ "two": 2 }"#;

        let mut assembler = Assembler::new();
        let mut value: Option<IntMap> = assembler.add(&chunk1);
        assert!(value.is_none());

        value = assembler.add(&chunk2);
        assert!(value.is_some());
        assert_eq!(value.unwrap().get("one"), Some(&1i32));

        value = assembler.add(&chunk3);
        assert!(value.is_some());
        assert_eq!(value.unwrap().get("two"), Some(&2i32));
    }
}