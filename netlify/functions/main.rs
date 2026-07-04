mod models;
mod search;
mod api;

use models::*;
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("embedded_ipc.json").unwrap();

    let laws: Vec<EmbeddedLaw> = serde_json::from_str(&data).unwrap();

    api::start_server(laws).await
}

