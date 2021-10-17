// See: https://github.com/Keats/rust-bcrypt
extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_pass(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_hash(password: &str, hash: &str) -> bool {
    verify(password, &hash).unwrap()
}
