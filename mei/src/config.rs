use std::env;

pub(crate) struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn load() -> Self {
        Self {
            verbose: env::var("MAI_VERBOSE").is_ok_and(|v| v == "1"),
        }
    }
}
