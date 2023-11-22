use {
    crate::{
        config::{Config, Verbose},
        log::Log,
        tool::Tools,
        vars::Vars,
    },
    std::sync::OnceLock,
};

pub(crate) struct Mei {
    verbose: Verbose,
    log: Log,
    tools: Tools,
    vars: Vars,
}

impl Mei {
    fn new() -> Self {
        let conf = Config::load();

        Self {
            verbose: conf.verbose,
            log: match Log::new() {
                Ok(log) => log,
                Err(err) => panic!("failed to create the log: {err}"),
            },
            tools: conf.tools,
            vars: Vars::new(),
        }
    }

    pub fn get() -> &'static Self {
        static MEI: OnceLock<Mei> = OnceLock::new();

        MEI.get_or_init(Self::new)
    }

    pub fn verbose(&self) -> bool {
        self.verbose.enabled()
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn tools(&self) -> &Tools {
        &self.tools
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }
}
