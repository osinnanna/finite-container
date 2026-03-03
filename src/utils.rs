use rand::{rng, RngExt}; 
use rand::distr::Alphanumeric;

pub fn generate_code(length: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

