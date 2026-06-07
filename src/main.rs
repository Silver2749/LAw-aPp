mod models;
mod search;

use models::*;

use std::fs;
use std::io;

fn main() {
    let data = fs::read_to_string("embedded_ipc.json").unwrap();

    let laws: Vec<EmbeddedLaw> = serde_json::from_str(&data).unwrap();

    println!("Describe an activity:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    search::search(input.trim().to_string(), &laws);
}

