use std::env;

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        // ensure that environment variables are loaded
        ensure_load_env();
        Self {}
    }
}

/// Loads the environment variables from the .env file
fn ensure_load_env() {
    println!("ENV loaded from .env");
    // ok to unwrap, if it fails, we should know
    dotenvy::dotenv().unwrap();
}

/// Loads specific variable from environment
///
/// Panics when expected variable is missing
fn try_from_env(name: &str) -> String {
    let err = format!("Expected {name} in the environment");
    env::var(name).expect(&err)
}
