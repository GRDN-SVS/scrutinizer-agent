extern crate actix;
extern crate actix_rt;
extern crate actix_web;
extern crate serde;
extern crate tokio;

#[macro_use]
extern crate diesel;
extern crate bson;
extern crate dotenv;
extern crate mongodb;
extern crate reqwest;
extern crate sodiumoxide;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::process;
use std::sync::Arc;

mod config;
mod crypto;
mod database;
mod handlers;
mod models;
mod services;

use database::executor::DBExecutor;
use database::mongo::MongoExecutor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if sodiumoxide::init().is_err() {
        eprintln!("ERROR: Could not initialize sodiumoxide correctly!");
        process::exit(1);
    }

    let addr = actix::SyncArbiter::start(2, || {
        DBExecutor::new(&env::var("DATABASE_URL").expect("No DATABASE_URL in .env"))
    });

    let encrypter = Arc::new(crypto::Decrypter::new());
    let mongo_exec = Arc::new(
        MongoExecutor::new(&env::var("MONGODB_URL").expect("No MONGODB_URL in .env")).await,
    );

    HttpServer::new(move || {
        App::new()
            .data(config::State { db: addr.clone() })
            .data(Arc::clone(&encrypter))
            .data(Arc::clone(&mongo_exec))
            .service(handlers::public_key::public_key)
            .service(handlers::scrutiny::decrypt_and_count_votes)
    })
    .bind(format!(
        "{}:8080",
        &env::var("APP_URL").expect("No APP_URL in .env")
    ))?
    .run()
    .await
}
