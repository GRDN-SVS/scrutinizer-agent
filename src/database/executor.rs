use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use diesel::Connection;

use crate::models;

/// An [actor](https://en.wikipedia.org/wiki/Actor_model)
/// that connects to a postgres database, being the only one
/// in charge of interacting directly with it.
pub struct DBExecutor(PgConnection);

impl DBExecutor {
    pub fn new(connection_string: &str) -> DBExecutor {
        DBExecutor(
            diesel::pg::PgConnection::establish(connection_string)
                .expect("Could not Connect to the database"),
        )
    }
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

/// Message that can be sent to the DBExecutor to
/// tell it to get all votes from the database
/// TODO: DO NOT LOAD EVERY SINGLE VOTE AT ONCE
pub struct GetVotes {}

impl Message for GetVotes {
    type Result = Result<Vec<models::Vote>, diesel::result::Error>;
}

impl Handler<GetVotes> for DBExecutor {
    type Result = Result<Vec<models::Vote>, diesel::result::Error>;

    fn handle(&mut self, _msg: GetVotes, _: &mut Self::Context) -> Self::Result {
        let query_string = "SELECT votes.id, votes.encrypted_vote, nonces.nonce, votes.voter_public_key FROM votes INNER JOIN nonces ON votes.nonce_id = nonces.id";
        let votes_ref: &Vec<models::Vote> = &diesel::sql_query(query_string).load(&self.0)?;
        let votes_obj: Vec<models::Vote> = votes_ref.clone();

        Ok(votes_obj)
    }
}
