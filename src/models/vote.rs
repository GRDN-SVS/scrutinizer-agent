use diesel::sql_types::{Binary, Integer};

// Representation of a Vote already stored inside the database
#[derive(Queryable, QueryableByName, Clone)]
pub struct Vote {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Binary"]
    pub encrypted_vote: Vec<u8>,
    #[sql_type = "Binary"]
    pub nonce: Vec<u8>,
    #[sql_type = "Binary"]
    pub voter_public_key: Vec<u8>,
}
