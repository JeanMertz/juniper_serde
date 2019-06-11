#![deny(unsafe_code)]

use juniper::ID;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(value: &ID, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&**value)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ID, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer)?;
    Ok(ID::new(string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct Test(#[serde(with = "self")] pub ID);

    #[test]
    fn test_serialize() {
        let test = Test(ID::new("1"));
        let json = json!({ "id": test });

        assert_eq!(r#"{"id":"1"}"#, json.to_string())
    }

    #[test]
    fn test_deserialize() {
        let test: Test = serde_json::from_str(r#""1""#).unwrap();
        let id = ID::new("1");

        assert_eq!(test.0, id)
    }
}
