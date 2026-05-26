use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use std::env;

fn main() {
    let secret = env::args()
        .nth(1)
        .expect("Usage: cargo run --bin hash_secret -- <secret>");

    let params = Params::new(
        19 * 1024,
        2,
        1,
        Some(32),
    )
    .unwrap();

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(secret.as_bytes(), &salt)
        .expect("failed to hash secret")
        .to_string();

    println!("{hash}");
}