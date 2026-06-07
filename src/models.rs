use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Law {
    pub chapter: u32,
    pub chapter_title: String,

    #[serde(rename = "Section", deserialize_with = "deserialize_section")]
    pub section: String,

    pub section_title: String,
    pub section_desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedLaw {
    pub section: String,
    pub title: String,
    pub description: String,
    pub embedding: Vec<f32>,
}

fn deserialize_section<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("Invalid section")),
    }
}
