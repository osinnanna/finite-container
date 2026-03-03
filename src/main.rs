use actix_web::{web, App, HttpServer};

mod handlers;
mod utils;
mod storage;

const PORT: u16 = 3000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:{}", PORT);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::hello))
            .route("/create", web::get().to(handlers::create_container))
            .route("/upload/{code}", web::post().to(handlers::upload_file))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
