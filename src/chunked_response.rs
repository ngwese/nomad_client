use serde::de::DeserializeOwned;
use serde_json;

pub struct Assembler {
    buffered: Option<String>,
}

impl Assembler {
    pub fn new() -> Self {
        Self { buffered: None }
    }

    pub fn add<T>(&mut self, chunk: &str) -> Result<Option<T>, serde_json::error::Error>
    where
        T: DeserializeOwned,
    {
        if self.buffered.is_none() {
            match serde_json::from_str::<T>(chunk) {
                Ok(value) => return Ok(Some(value)),
                Err(e) => {
                    if e.is_eof() {
                        self.buffered = Some(String::from(chunk));
                        return Ok(None);
                    } else {
                        return Err(e);
                    }
                }
            }
        };

        // Optimistic deserialize failed or have an existing buffer
        if let Some(ref mut buffer) = self.buffered {
            buffer.push_str(chunk);
            if let Ok(value) = serde_json::from_str::<T>(&buffer) {
                self.buffered = None;
                return Ok(Some(value));
            };
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    type IntMap = HashMap<String, i32>;
    type StringMap = HashMap<String, String>;

    #[test]
    fn complete_object() {
        let chunk1 = r#"{ "one": 1 }"#;

        let mut assembler = Assembler::new();
        let value = assembler
            .add::<IntMap>(&chunk1)
            .expect("deserialized value");
        assert_eq!(value.unwrap().get("one"), Some(&1i32));
    }

    #[test]
    fn partial_object() {
        let chunk1 = r#"{ "one": 1"#;
        let chunk2 = r#"}"#;
        let chunk3 = r#"{ "two": 2 }"#;

        let mut assembler = Assembler::new();
        let value = assembler.add::<IntMap>(&chunk1);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), None);

        if let Ok(Some(object)) = assembler.add::<IntMap>(&chunk2) {
            assert_eq!(object.get("one"), Some(&1i32));
        } else {
            panic!("expected completed parse");
        }

        if let Ok(Some(object)) = assembler.add::<IntMap>(&chunk3) {
            assert_eq!(object.get("two"), Some(&2i32));
        } else {
            panic!("expected completed parse");
        }
    }

    #[test]
    fn syntax_error() {
        let chunk1 = r#"{ "two": 1 }"#;
        let mut assembler = Assembler::new();

        let value = assembler.add::<StringMap>(&chunk1);
        assert!(value.is_err());
        println!("Value: {:?}", value);
    }
}
