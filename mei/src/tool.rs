use {
    crate::{
        mei::Mei,
        spawn::{self, Info, Spawn},
    },
    std::{
        borrow::Cow,
        ffi::OsStr,
        io::ErrorKind,
        process::{Command, Stdio},
    },
};

pub fn tool<S>(name: S) -> Tool
where
    S: AsRef<OsStr>,
{
    Tool(Command::new(name))
}

#[must_use]
pub struct Tool(Command);

impl Tool {
    pub fn name(&self) -> Cow<str> {
        self.0.get_program().to_string_lossy()
    }

    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.0.arg(arg);
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    pub fn stdout<S>(&mut self, stdout: S) -> &mut Self
    where
        S: Into<Stdio>,
    {
        self.0.stdout(stdout);
        self
    }

    pub fn stderr<S>(&mut self, stderr: S) -> &mut Self
    where
        S: Into<Stdio>,
    {
        self.0.stderr(stderr);
        self
    }

    pub fn into_command(self) -> Command {
        self.0
    }

    pub fn spawn(&mut self) {
        Spawn::spawn(self);
    }
}

impl Spawn for Tool {
    fn spawn(&mut self) {
        match spawn::spawn_process(&mut self.0, Info::Running) {
            Ok(()) => return,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let name = self.name();
                Mei::get().install(&name);
            }
            Err(err) => {
                let name = self.name();
                panic!("failed to spawn {name} process: {err}");
            }
        }

        if let Err(err) = spawn::spawn_process(&mut self.0, Info::Running) {
            let name = self.name();
            panic!("failed to spawn {name} process: {err}");
        }
    }
}
