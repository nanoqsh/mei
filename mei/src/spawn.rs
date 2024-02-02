use {
    crate::mei::Mei,
    std::{ffi::OsStr, fmt, io, process::Command},
};

#[derive(Clone, Copy)]
pub enum Info<'a> {
    Running,
    Building { name: &'a str },
    Installing { name: &'a str },
}

impl Info<'_> {
    fn log(self, cmd: &Command) {
        let mei = Mei::get();
        let log = mei.log();
        _ = match self {
            Self::Running => log.running(&DisplayCommand {
                cmd,
                name: "",
                verbose: mei.verbose(),
            }),
            Self::Building { name } => log.building(&DisplayCommand {
                cmd,
                name,
                verbose: mei.verbose(),
            }),
            Self::Installing { name } => log.installing(&DisplayCommand {
                cmd,
                name,
                verbose: mei.verbose(),
            }),
        };
    }
}

/// The trait for process spawn.
pub trait Spawn {
    /// Spawns a new process.
    fn spawn(&mut self);
}

impl Spawn for Command {
    fn spawn(&mut self) {
        if let Err(err) = spawn_process(self, Info::Running) {
            let name = self.get_program().to_string_lossy();
            panic!("failed to spawn {name} process: {err}");
        }
    }
}

pub fn spawn_process(cmd: &mut Command, info: Info) -> io::Result<()> {
    let child = cmd.spawn()?;

    info.log(cmd);
    let out = match child.wait_with_output() {
        Ok(out) => out,
        Err(err) => {
            let name = cmd.get_program().to_string_lossy();
            panic!("failed to wait the output from {name} process: {err}");
        }
    };

    if out.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&out.stderr);
    panic!("run failed:\n{stderr}\n");
}

struct DisplayCommand<'a> {
    cmd: &'a Command,
    name: &'a str,
    verbose: bool,
}

impl fmt::Display for DisplayCommand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name;
        if self.verbose || name.trim().is_empty() {
            let proc = self.cmd.get_program().to_string_lossy();
            write!(f, "{proc}")?;
        } else {
            write!(f, "{name}")?;
        }

        if self.verbose {
            for arg in self.cmd.get_args().map(OsStr::to_string_lossy) {
                write!(f, " {arg}")?;
            }
        }

        Ok(())
    }
}
