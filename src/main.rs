use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct Law {
    keywords: Vec<String>,
    law: String,
    penalty: String,
}

fn keyword_score(input: &str, keyword: &str) -> usize {
    let input_words: Vec<&str> = input.split_whitespace().collect();
    let keyword_words: Vec<&str> = keyword.split_whitespace().collect();

    keyword_words
        .iter()
        .filter(|word| input_words.contains(word))
        .count()
}

fn main() {
    let data = fs::read_to_string("src/laws.json").expect("Failed to read laws.json");

    let laws: Vec<Law> = serde_json::from_str(&data).expect("Invalid JSON");

    println!("Enter an activity:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim().to_lowercase();

    let matcher = SkimMatcherV2::default();

    let mut best_score = 0;
    let mut best_match = None;

    for law in &laws {
        for keyword in &law.keywords {
            if let Some(score) = matcher.fuzzy_match(&input, keyword) {
                if score > best_score {
                    best_score = score;
                    best_match = Some(law);
                }
            }
        }
    }
    let mut best_score = 0;
    let mut best_match = None;

    for law in &laws {
        for keyword in &law.keywords {
            let score = keyword_score(&input, keyword);

            if score > best_score {
                best_score = score;
                best_match = Some(law);
            }
        }
    }
    match best_match {
        Some(law) if best_score > 0 => {
            println!("\nPossible Match Found");
            println!("Law Broken: {}", law.law);
            println!("Penalty: {}", law.penalty);
            println!("Score: {}", best_score);
        }
        _ => {
            println!("No matching law found.");
        }
    }
}
