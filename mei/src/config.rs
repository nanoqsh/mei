use {
    crate::{env, tool::Tools},
    std::{fs, path::PathBuf},
    toml::{Document, Item, TableLike, Value},
};

pub(crate) struct Config {
    pub log: Log,
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

        let log = Log::from_var()
            .or_else(|| Log::from_toml(mei?.get("log")?.as_table_like()?))
            .unwrap_or_default();

        let verbose = Verbose::from_var()
            .or_else(|| Verbose::from_toml(mei?.get("verbose")?.as_value()?))
            .unwrap_or_default();

        let tools = mei
            .and_then(|m| Some(Tools::from_toml(m.get("tools")?.as_table_like()?)))
            .unwrap_or_default();

        Self {
            log,
            verbose,
            tools,
        }
    }
}

pub(crate) enum Log {
    #[cfg(unix)]
    Console,
    Path(PathBuf),
}

impl Log {
    fn default_path() -> Self {
        Self::Path(PathBuf::from("log"))
    }

    fn from_var() -> Option<Self> {
        let val = env::try_var(env::LOG)?;
        match &val[..] {
            "console" => Some(Self::default()),
            path => Some(Self::Path(PathBuf::from(path))),
        }
    }

    fn from_toml(table: &dyn TableLike) -> Option<Self> {
        let console = table.get("console").and_then(Item::as_bool);
        let path = table.get("path").and_then(Item::as_str);
        match (console, path) {
            (Some(true), _) => Some(Self::default()),
            (Some(false), _) => Some(Self::default_path()),
            (_, Some(path)) => Some(Self::Path(PathBuf::from(path))),
            _ => None,
        }
    }
}

impl Default for Log {
    fn default() -> Self {
        #[cfg(unix)]
        {
            Self::Console
        }

        #[cfg(not(unix))]
        {
            Self::default_path()
        }
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
        let val = env::try_var(env::VERBOSE)?;
        match &val[..] {
            "0" | "false" => Some(Self::No),
            "1" | "true" => Some(Self::Yes),
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
