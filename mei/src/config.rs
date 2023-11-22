use {
    crate::tool::Tools,
    std::{env, fs},
    toml::{Document, Value},
};

pub(crate) struct Config {
    pub verbose: Verbose,
    pub tools: Tools,
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

        let verbose = Verbose::from_var()
            .or_else(|| Verbose::from_toml(mei?.get("verbose")?.as_value()?))
            .unwrap_or_default();

        let tools = mei
            .and_then(|m| Some(Tools::from_toml(m.get("tools")?.as_table_like()?)))
            .unwrap_or_default();

        Self { verbose, tools }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) enum Verbose {
    #[default]
    No,
    Yes,
    Full,
}

impl Verbose {
    fn from_var() -> Option<Self> {
        let val = env::var("MAI_VERBOSE").ok()?;
        match &val[..] {
            "0" => Some(Self::No),
            "1" => Some(Self::Yes),
            "full" => Some(Self::Full),
            _ => None,
        }
    }

    fn from_toml(val: &Value) -> Option<Self> {
        match val {
            Value::String(s) if s.value() == "full" => Some(Self::Full),
            Value::Boolean(f) if *f.value() => Some(Self::Yes),
            Value::Boolean(_) => Some(Self::No),
            _ => None,
        }
    }

    pub fn enabled(self) -> bool {
        matches!(self, Self::Yes | Self::Full)
    }
}
