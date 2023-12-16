use {
    crate::{
        config::{Config, Verbose},
        env,
        log::Log,
        tool::Tools,
        var::Vars,
    },
    std::sync::OnceLock,
};

static MEI: OnceLock<Mei> = OnceLock::new();

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
        let vars = Vars::new();

        Self {
            log: Log::new(conf.log, &vars),
            vars,
            tools: conf.tools,
            verbose: conf.verbose,
        }
    }

    pub fn try_get() -> Option<&'static Self> {
        MEI.get()
    }

    pub fn get() -> &'static Self {
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
