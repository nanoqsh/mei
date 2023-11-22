use {
    crate::{config::Config, log::Log, vars::Vars},
    std::sync::OnceLock,
};

pub(crate) struct Mei {
    verbose: bool,
    log: Log,
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
            vars: Vars::new(),
        }
    }

    pub fn get() -> &'static Self {
        static MEI: OnceLock<Mei> = OnceLock::new();

        MEI.get_or_init(Self::new)
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }

    pub fn install(&self, tool: &str) {
        todo!("install {tool}")
    }
}
