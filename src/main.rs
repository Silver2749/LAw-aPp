use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::{Deserialize, Deserializer};
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct Law {
    chapter: u32,
    chapter_title: String,

    #[serde(rename = "Section", deserialize_with = "deserialize_section")]
    section: String,

    section_title: String,
    section_desc: String,
}

fn deserialize_section<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("Expected string or number")),
    }
}

fn main() {
    let data = fs::read_to_string("ipc.json").expect("Failed to read JSON");

    let laws: Vec<Law> = serde_json::from_str(&data).expect("Invalid JSON");

    println!("Describe an activity:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let query = input.trim().to_lowercase();

    let matcher = SkimMatcherV2::default();

    let mut results = Vec::new();

    for law in &laws {
        let title_score = matcher
            .fuzzy_match(&law.section_title.to_lowercase(), &query)
            .unwrap_or(0);

        let desc_score = matcher
            .fuzzy_match(&law.section_desc.to_lowercase(), &query)
            .unwrap_or(0);

        let score = title_score.max(desc_score);

        if score > 30 {
            results.push((score, law));
        }
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));

    if results.is_empty() {
        println!("No matching laws found.");
        return;
    }

    println!("\nTop Matches:");

    for (score, law) in results.iter().take(5) {
        println!("\n========================");
        println!("Confidence: {}", score);
        println!("Section {}", law.section);
        println!("Title: {}", law.section_title);
        println!("Chapter: {}", law.chapter_title);
        println!("\n{}", law.section_desc);
    }
}
