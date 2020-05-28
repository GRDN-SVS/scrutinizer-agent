use serde::Serialize;

#[derive(Serialize)]
pub struct ScrutinyError {
    pub code: i32,
    pub error: String,
}

#[derive(Serialize)]
pub struct MongoError {
    pub code: i32,
    pub error: String,
}
