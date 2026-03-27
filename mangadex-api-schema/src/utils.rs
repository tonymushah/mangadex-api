use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MangadexMapRepr<K, V>
where
    K: Eq + Hash,
{
    EmptyVec([(); 0]),
    Map(HashMap<K, V>),
}

// There is [PHP Bug](https://discord.com/channels/403905762268545024/841005104362160168/1131502086425673778) that transforms empty maps into array.
//
// This is here to fix that when deserializing things.
pub fn deserialize_map_because_of_md_bug<'de, D, K, V>(
    deserializer: D,
) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    let repr: MangadexMapRepr<K, V> = MangadexMapRepr::deserialize(deserializer)?;
    match repr {
        MangadexMapRepr::EmptyVec(_) => Ok(Default::default()),
        MangadexMapRepr::Map(v) => Ok(v),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde_json::{from_value, json};

    #[derive(Debug, Deserialize)]
    struct TestMapStruct {
        #[serde(deserialize_with = "super::deserialize_map_because_of_md_bug")]
        data: HashMap<String, String>,
    }
    #[test]
    fn test_md_repr_map_empty_array() -> anyhow::Result<()> {
        let aaa: TestMapStruct = from_value(json!({
            "data": []
        }))?;
        assert_eq!(aaa.data.len(), 0);
        Ok(())
    }
    #[test]
    fn test_md_repr_map_with_val() -> anyhow::Result<()> {
        let aaa: TestMapStruct = from_value(json!({
            "data": {
                "aaad": "ve"
            }
        }))?;
        assert_eq!(aaa.data.len(), 1);
        assert_eq!(aaa.data["aaad"], "ve");
        Ok(())
    }
}
