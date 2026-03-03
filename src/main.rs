use actix_web::{web, App, HttpServer};

mod handlers;
mod utils;

const PORT: u16 = 3000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:{}", PORT);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::hello))
            .route("/code", web::get().to(handlers::get_code))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
