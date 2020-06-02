use serde::Serialize;

#[derive(Serialize)]
pub struct ScrutinyResult {
    pub option_id: i32,
    pub result: i32,
}

#[derive(Serialize)]
pub struct InsertableResults {
    pub results: Vec<ScrutinyResult>,
}
