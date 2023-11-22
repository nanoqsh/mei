use {
    std::{env, fs},
    toml::Document,
};

pub(crate) struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn load() -> Self {
        let doc: Document = {
            let path = "Cargo.toml";
            let content = match fs::read_to_string(path) {
                Ok(content) => content,
                Err(err) => panic!("failed to read {path}: {err}"),
            };

            match content.parse() {
                Ok(doc) => doc,
                Err(err) => panic!("failed to parse {path}: {err}"),
            }
        };

        let mei = doc
            .get("package")
            .or_else(|| doc.get("workspace"))
            .and_then(|p| p.get("metadata")?.get("mei"));

        let verbose = var_as_bool("MAI_VERBOSE")
            .or_else(|| mei?.get("verbose")?.as_bool())
            .unwrap_or_default();

        Self { verbose }
    }
}

fn var_as_bool(key: &str) -> Option<bool> {
    let val = env::var(key).ok()?;
    match &val[..] {
        "0" => Some(false),
        "1" => Some(true),
        _ => None,
    }
}
