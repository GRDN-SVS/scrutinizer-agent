use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

use super::messages;
use crate::crypto;

#[get("/boxPublicKey")]
pub async fn public_key(decrypter: web::Data<Arc<crypto::Decrypter>>) -> impl Responder {
    let key = messages::success::PublicKey {
        code: 200,
        key: &decrypter.box_public_key.0.to_vec(),
    };

    HttpResponse::Ok().json(key)
}
