use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
    os::unix::process,
    process::Stdio,
};

pub(crate) struct Log {
    out: File,
}

impl Log {
    pub fn new() -> io::Result<Self> {
        let parent_id = process::parent_id();
        let out = File::options()
            .append(true)
            .open(format!("/proc/{parent_id}/fd/2"))?;

        Ok(Self { out })
    }

    pub fn building(&self, s: &dyn Display) -> io::Result<()> {
        self.write("    Building", s)
    }

    pub fn running(&self, s: &dyn Display) -> io::Result<()> {
        self.write("     Running", s)
    }

    fn write(&self, label: &str, s: &dyn Display) -> io::Result<()> {
        let mut out = &self.out;
        writeln!(out, "\x1b[2K\r\x1b[1;34m{label}\x1b[0m {s}")?;
        out.flush()
    }

    pub fn stdio(&self) -> io::Result<Stdio> {
        let out = self.out.try_clone()?;
        Ok(Stdio::from(out))
    }
}
