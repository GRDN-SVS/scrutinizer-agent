use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Arc;

use super::messages;
use crate::{
    config::State,
    crypto,
    database::{executor::GetVotes, mongo},
    models::{InsertableResults, ScrutinyResult, Vote},
    services::JudgeService,
};

#[get("/countVotes")]
pub async fn decrypt_and_count_votes(
    state: web::Data<State>,
    decrypter: web::Data<Arc<crypto::Decrypter>>,
    mongo_exec: web::Data<Arc<mongo::MongoExecutor>>,
) -> impl Responder {
    let db_executor = &state.db;

    let mut votes: Vec<Vote> = match db_executor.send(GetVotes {}).await.unwrap() {
        Ok(votes_vec) => votes_vec,
        Err(_) => {
            return HttpResponse::Ok().json(messages::error::ScrutinyError {
                code: 500,
                error: String::from("Could not load the votes from the database"),
            })
        }
    };
    
    let judge_service = match JudgeService::new().await {
        Ok(instance) => instance,
        Err(_) => {
            return HttpResponse::Ok().json(messages::error::ScrutinyError {
                code: 404,
                error: String::from("Judge Not Found!"),
            })
        }
    };
    // Decrypt the votes by verifying the judge's signature,
    // then open the secret shared with the judge and the client
    let mut election_results: HashMap<u8, i32> = HashMap::new();
    for vote in &mut votes {
        vote.encrypted_vote =
            decrypter.open(&vote.encrypted_vote, &vote.nonce, &judge_service.box_public_key);
        vote.encrypted_vote = decrypter.verify(&vote.encrypted_vote, &judge_service.sign_public_key);
        let original_vote =
            decrypter.open(&vote.encrypted_vote, &vote.nonce, &vote.voter_public_key);

        if let Some(curr_count) = election_results.get_mut(&original_vote[0]) {
            *curr_count += 1;
        } else {
            election_results.insert(original_vote[0], 1);
        }
    }
    // Store in MongoDB
    let mut insertable_res_vec = Vec::with_capacity(election_results.len());
    for (option, count) in election_results.into_iter() {
        let option_result = ScrutinyResult {
            option_id: option as i32,
            result: count,
        };
        insertable_res_vec.push(option_result);
    }
    let insertable_results = InsertableResults {
        results: insertable_res_vec,
    };

    match mongo_exec.save_results(&insertable_results).await {
        Ok(_) => {
            HttpResponse::Ok().json(messages::success::ScrutinyFinished {
                code: 200,
                success: String::from("Scrutiny process finished successfully"),
            })
        }
        Err(_) => {
            HttpResponse::Ok().json(messages::error::MongoError {
                code: 500,
                error: String::from("Could not save the results to mongodb!"),
            })
        }
    }
}
