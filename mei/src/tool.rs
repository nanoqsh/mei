use {crate::spawn::Spawn, std::ffi::OsStr};

pub fn tool<N>(name: N) -> Tool
where
    N: Into<Box<str>>,
{
    Tool::new(name.into())
}

pub struct Tool {
    name: Box<str>,
    args: Vec<Box<OsStr>>,
}

impl Tool {
    fn new(name: Box<str>) -> Self {
        Self { name, args: vec![] }
    }

    pub fn arg<S>(mut self, arg: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        self.args.push(Box::from(arg.as_ref()));
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|arg| Box::from(arg.as_ref())));

        self
    }
}

impl Spawn for Tool {
    fn spawn(&mut self) {
        use std::process::{Command, Stdio};

        let Self { name, args } = self;
        let mut proc = Command::new(&name[..]);
        for arg in args {
            proc.arg(arg);
        }

        proc.stdout(Stdio::piped()).stderr(Stdio::piped());
        Spawn::spawn(&mut proc);
    }
}
