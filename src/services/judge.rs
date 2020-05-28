use std::env;

pub struct JudgeService {
    pub public_key: Vec<u8>,
}

impl JudgeService {
    pub async fn new() -> Result<JudgeService, reqwest::Error> {
        let public_key_str = reqwest::get(&format!(
            "{}/publicKey",
            env::var("JUDGE_URL").expect("No JUDGE_URL set in .env")
        ))
        .await?
        .text()
        .await?;

        let mut public_key_vec: Vec<u8> = Vec::with_capacity(32);
        for number in public_key_str[1..public_key_str.len() - 1].chars() {
            public_key_vec.push(number.to_digit(10).unwrap() as u8);
        }

        Ok(JudgeService {
            public_key: public_key_vec,
        })
    }
}
