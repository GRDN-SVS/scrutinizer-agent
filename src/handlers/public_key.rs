use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

use super::messages;
use crate::crypto;

#[get("/publicKey")]
pub async fn public_key(encrypter: web::Data<Arc<crypto::Decrypter>>) -> impl Responder {
    let key = messages::success::PublicKey {
        code: 200,
        key: &encrypter.public_key,
    };

    HttpResponse::Ok().json(key)
}
