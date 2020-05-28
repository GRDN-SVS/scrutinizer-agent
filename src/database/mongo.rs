use mongodb::{
    error::{Error, ErrorKind},
    options::ClientOptions,
    Client, Database,
};
use std::sync::Arc;

use crate::models;

/// Connects to a mongodb database, being the only one
/// in charge of interacting directly with it.
pub struct MongoExecutor(Database);

impl MongoExecutor {
    pub async fn new(connection_string: &str) -> MongoExecutor {
        let client_options = ClientOptions::parse(connection_string)
            .await
            .expect("Could not parse the Mongo connection string");
        let client =
            Client::with_options(client_options).expect("Could not connect to the Mongo Database");

        MongoExecutor(client.database("elections_db"))
    }

    pub async fn save_results(
        &self,
        results: &models::InsertableResults,
    ) -> Result<(), mongodb::error::Error> {
        let collection = &self.0.collection("results");
        match bson::to_bson(results)? {
            bson::Bson::Document(doc_model) => {
                collection.insert_one(doc_model, None).await?;
            }
            _ => {
                return Err(Error {
                    kind: Arc::new(ErrorKind::InternalError {
                        message: String::from("Failed to create Document"),
                    }),
                })
            }
        }
        Ok(())
    }
}
