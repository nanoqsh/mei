use crate::{
    config::{Config, Verbose},
    env,
    log::Log,
    tool::Tools,
    var::Vars,
};

pub(crate) struct Mei {
    log: Log,
    vars: Vars,
    tools: Tools,
    verbose: Verbose,
}

impl Mei {
    fn new() -> Self {
        env::rerun_if_env_changed();
        let conf = Config::load();

        Self {
            log: Log::new(conf.log),
            vars: Vars::new(),
            tools: conf.tools,
            verbose: conf.verbose,
        }
    }

    pub fn get() -> &'static Self {
        use std::sync::OnceLock;

        static MEI: OnceLock<Mei> = OnceLock::new();

        MEI.get_or_init(Self::new)
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }

    pub fn tools(&self) -> &Tools {
        &self.tools
    }

    pub fn verbose(&self) -> bool {
        self.verbose.enabled()
    }
}
