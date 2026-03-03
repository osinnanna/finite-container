use actix_web::{Responder, HttpResponse, web};

use crate::utils;
use crate::storage;

const CODE_LENGTH: usize = 8;
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello From Finite Container\n")
}
pub async fn create_container() -> impl Responder {
    let code = utils::generate_code(CODE_LENGTH);
    // Code and Date with expiry should be sent to DB");
    HttpResponse::Ok().body(format!("Your container code is: {}\n", code))
}
pub async fn upload_file(path: web::Path<String>, body: web::Bytes) -> impl Responder {
    let container_id = path.into_inner();
    let filename = "uploaded file";
    match storage::save_file(&container_id, filename, &body) {
        Ok(_) => HttpResponse::Ok().body("File uploaded\n"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}\n", e)),
    }
}
