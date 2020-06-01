use std::env;

pub struct JudgeService {
    pub box_public_key: Vec<u8>,
    pub sign_public_key: Vec<u8>,
}

impl JudgeService {
    pub async fn new() -> Result<JudgeService, reqwest::Error> {
        let box_public_key_str = reqwest::get(&format!(
            "{}/boxPublicKey",
            &env::var("JUDGE_URL").expect("No JUDGE_URL set in .env")
        ))
        .await?
        .text()
        .await?;

        let sign_public_key_str = reqwest::get(&format!(
            "{}/signPublicKey",
            &env::var("JUDGE_URL").expect("No JUDGE_URL set in .env")
        ))
        .await?
        .text()
        .await?;

        let mut box_public_key_vec: Vec<u8> = Vec::with_capacity(32);
        let mut sign_public_key_vec: Vec<u8> = Vec::with_capacity(32);
        
        let box_numbers: Vec<&str> = box_public_key_str.split('[').collect();
        let box_numbers: Vec<&str> = box_numbers[1].split(']').collect();
        let box_numbers: Vec<&str> = box_numbers[0].split(',').collect();
        let sign_numbers: Vec<&str> = sign_public_key_str.split('[').collect();
        let sign_numbers: Vec<&str> = sign_numbers[1].split(']').collect();
        let sign_numbers: Vec<&str> = sign_numbers[0].split(',').collect();

        for number in box_numbers.iter() {
            box_public_key_vec.push(number.parse::<u8>().unwrap());
        }
        for number in sign_numbers.iter() {
            sign_public_key_vec.push(number.parse::<u8>().unwrap());
        }

        Ok(JudgeService {
            box_public_key: box_public_key_vec,
            sign_public_key: sign_public_key_vec,
        })
    }
}
