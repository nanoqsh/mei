pub(crate) const VERBOSE: &str = "MAI_VERBOSE";

pub(crate) fn rerun_if_env_changed() {
    println!("cargo:rerun-if-env-changed={VERBOSE}");
}

pub(crate) fn var(key: &str) -> String {
    match try_var(key) {
        Some(var) => var,
        None => panic!("the {key} variable should be set"),
    }
}

pub(crate) fn try_var(key: &str) -> Option<String> {
    use std::env::{self, VarError};

    match env::var(key) {
        Ok(var) => Some(var),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(var)) => {
            panic!("the {key} variable should be utf-8 encoded, but {var:?} is not")
        }
    }
}
