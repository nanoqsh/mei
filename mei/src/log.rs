use {
    crate::config::Log as Config,
    std::{
        fmt::Display,
        fs::File,
        io::{self, IsTerminal, Write},
        os::unix::process,
        process::Stdio,
    },
};

pub(crate) struct Log {
    out: File,
}

impl Log {
    pub fn new(conf: Config) -> Self {
        let res = match conf {
            #[cfg(unix)]
            Config::Console => {
                let parent_id = process::parent_id();
                File::options()
                    .append(true)
                    .open(format!("/proc/{parent_id}/fd/2"))
            }
            Config::Path(path) => File::create(path),
        };

        match res {
            Ok(out) => Self { out },
            Err(err) => panic!("failed to create the log: {err}"),
        }
    }

    pub fn info(&self, s: &dyn Display) -> io::Result<()> {
        self.write("        Info", s)
    }

    pub fn installing(&self, s: &dyn Display) -> io::Result<()> {
        self.write("  Installing", s)
    }

    pub fn building(&self, s: &dyn Display) -> io::Result<()> {
        self.write("    Building", s)
    }

    pub fn running(&self, s: &dyn Display) -> io::Result<()> {
        self.write("     Running", s)
    }

    fn write(&self, label: &str, s: &dyn Display) -> io::Result<()> {
        let mut out = &self.out;
        if cfg!(unix) && out.is_terminal() {
            writeln!(out, "\x1b[2K\r\x1b[1;34m{label}\x1b[0m {s}")?;
        } else {
            writeln!(out, "{label} {s}")?;
        }

        out.flush()
    }

    pub fn stdio(&self) -> io::Result<Stdio> {
        let out = self.out.try_clone()?;
        Ok(Stdio::from(out))
    }
}
