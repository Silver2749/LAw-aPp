use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

use crate::models::*;

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot += a[i] * b[i];

        norm_a += a[i] * a[i];

        norm_b += b[i] * b[i];
    }

    dot / (norm_a.sqrt() * norm_b.sqrt())
}

pub fn search(query: String, laws: &Vec<EmbeddedLaw>) {
    let mut model =
        TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2)).unwrap();

    let query_vector = model.embed(vec![query], None).unwrap();

    let query_embedding = &query_vector[0];

    let mut scores = Vec::new();

    for law in laws {
        let similarity = cosine_similarity(query_embedding, &law.embedding);

        scores.push((similarity, law));
    }

    scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    println!("\nTop Matches:\n");

    for (score, law) in scores.iter().take(5) {
        println!("Similarity: {:.3}", score,);

        println!("Section {}", law.section,);

        println!("{}", law.title,);

        println!("{}", law.description,);

        println!("--------------------------------");
    }
}
