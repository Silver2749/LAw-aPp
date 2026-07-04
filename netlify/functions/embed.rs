use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

use crate::models::*;

pub fn create_embeddings(laws: &Vec<Law>) -> Vec<EmbeddedLaw> {
    let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2)).unwrap();

    let mut embedded = Vec::new();

    for law in laws {
        let document = format!(
            "
Chapter: {}

Section: {}

Title: {}

Description:
{}
",
            law.chapter_title, law.section, law.section_title, law.section_desc
        );

        let vector = model.embed(vec![document], None).unwrap();

        embedded.push(EmbeddedLaw {
            section: law.section.clone(),

            title: law.section_title.clone(),

            description: law.section_desc.clone(),

            embedding: vector[0].clone(),
        });
    }

    embedded
}
