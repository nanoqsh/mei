use {
    crate::mei::Mei,
    std::{ffi::OsStr, fmt, process::Command},
};

#[derive(Clone, Copy)]
pub enum Info<'a> {
    Running,
    Building { name: Option<&'a str> },
}

impl Info<'_> {
    fn log(self, cmd: &Command) {
        let mei = Mei::get();
        let log = mei.log();
        _ = match self {
            Self::Running => log.running(&DisplayCommand {
                cmd,
                name: None,
                verbose: mei.verbose(),
            }),
            Self::Building { name } => log.building(&DisplayCommand {
                cmd,
                name,
                verbose: mei.verbose(),
            }),
        };
    }
}

pub trait Spawn {
    fn spawn(&mut self);
}

impl Spawn for Command {
    fn spawn(&mut self) {
        spawn_process(self, Info::Running);
    }
}

pub fn spawn_process(cmd: &mut Command, info: Info) {
    info.log(cmd);
    match cmd.spawn() {
        Ok(child) => {
            let out = match child.wait_with_output() {
                Ok(out) => out,
                Err(err) => {
                    let name = cmd.get_program().to_string_lossy();
                    panic!("failed to wait the output from {name} process: {err}");
                }
            };

            if out.status.success() {
                return;
            }

            let stderr = String::from_utf8_lossy(&out.stderr);
            panic!("run failed:\n{stderr}\n");
        }
        Err(err) => {
            let name = cmd.get_program().to_string_lossy();
            panic!("failed to spawn {name} process: {err}");
        }
    }
}

struct DisplayCommand<'a> {
    cmd: &'a Command,
    name: Option<&'a str>,
    verbose: bool,
}

impl fmt::Display for DisplayCommand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name {
            Some(name) if self.verbose => {
                write!(f, "{name}: ")?;
                let proc = self.cmd.get_program().to_string_lossy();
                write!(f, "{proc}")?;
            }
            Some(name) => write!(f, "{name}")?,
            None => {
                let proc = self.cmd.get_program().to_string_lossy();
                write!(f, "{proc}")?;
            }
        }

        if self.verbose {
            for arg in self.cmd.get_args().map(OsStr::to_string_lossy) {
                write!(f, " {arg}")?;
            }
        }

        Ok(())
    }
}
