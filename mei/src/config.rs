use {crate::fs, std::env, toml::Document};

pub(crate) struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn load() -> Self {
        let doc: Document = {
            let path = "Cargo.toml";
            let content = fs::read_to_string(path);
            match content.parse() {
                Ok(doc) => doc,
                Err(err) => panic!("failed to parse {path:?}: {err}"),
            }
        };

        let mei = doc
            .get("package")
            .or_else(|| doc.get("workspace"))
            .and_then(|p| p.get("metadata")?.get("mei"));

        let verbose = env::var("MAI_VERBOSE")
            .ok()
            .map(|v| v == "1")
            .or_else(|| mei?.get("verbose")?.as_bool())
            .unwrap_or_default();

        Self { verbose }
    }
}
