use serde::Serialize;

#[derive(Serialize)]
pub struct ScrutinyResult {
    pub option_id: Vec<u8>,
    pub result: i32,
}

#[derive(Serialize)]
pub struct InsertableResults {
    pub results: Vec<ScrutinyResult>,
}
