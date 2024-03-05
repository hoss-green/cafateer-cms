use argon2::{password_hash::SaltString, *};
use rand::Rng;

pub fn generate_salt() -> String {
    let mut rnd_gen = rand::thread_rng();
    let mut numbers: Vec<String> = Vec::new();
    (0..20).to_owned().into_iter().for_each(|_| {
        let next = rnd_gen.gen_range(0..=9).to_string();
        numbers.push(next);
    });

    numbers.join("")
}

pub fn calculate_hash(password: &String, salt: &String) -> String {
    let salt_string: SaltString = match SaltString::from_b64(salt.as_str()) {
        Ok(salt) => salt,
        Err(__) => return "".to_string(),
    };
    let hash_method = Argon2::default();
    let combined_string = format!("{}{}", password, salt);
    let pw_hash = match hash_method.hash_password(combined_string.as_bytes(), &salt_string) {
        Ok(pw) => pw.to_string(),
        Err(err) => panic!("Could not generate hash: {}", err),
    };

    pw_hash
}

