use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_files::Files;
use crate::models::*;
use crate::search;

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub query: String,
}

pub async fn search_handler(
    query: web::Query<SearchQuery>,
    laws: web::Data<Vec<EmbeddedLaw>>,
) -> HttpResponse {
    let results = search::search_laws(query.query.clone(), &laws);
    HttpResponse::Ok().json(results)
}

pub async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

pub async fn start_server(laws: Vec<EmbeddedLaw>) -> std::io::Result<()> {
    let laws_data = web::Data::new(laws);

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(laws_data.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(
                web::scope("/api")
                    .route("/search", web::get().to(search_handler))
                    .route("/health", web::get().to(health_handler))
            )
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
