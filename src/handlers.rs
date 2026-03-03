use actix_web::{Responder, HttpResponse};

use crate::utils;

const CODE_LENGTH: usize = 8;
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello From Finite Container\n")
}
pub async fn get_code() -> impl Responder {
    let code = utils::generate_code(CODE_LENGTH); 
    HttpResponse::Ok().body(format!("Your code is: {}\n", code))
}
