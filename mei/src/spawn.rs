use {
    crate::mei::Mei,
    std::{ffi::OsStr, fmt, process::Command},
};

pub trait Spawn {
    fn spawn(&mut self);
}

impl Spawn for Command {
    fn spawn(&mut self) {
        let mei = Mei::get();
        let log = mei.log();
        _ = log.running(&FormatProc {
            cmd: self,
            verbose: mei.verbose(),
        });

        match self.spawn() {
            Ok(child) => {
                let out = match child.wait_with_output() {
                    Ok(out) => out,
                    Err(err) => panic!(
                        "failed to wait the output from {name:?} process: {err}",
                        name = self.get_program(),
                    ),
                };

                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    panic!("run failed:\n{stderr}\n");
                }
            }
            Err(err) => panic!(
                "failed to spawn {name:?} process: {err}",
                name = self.get_program(),
            ),
        }
    }
}

struct FormatProc<'a> {
    cmd: &'a Command,
    verbose: bool,
}

impl fmt::Display for FormatProc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proc = self.cmd.get_program();
        write!(f, "{proc}", proc = DisplayOsStr(proc))?;

        if self.verbose {
            for arg in self.cmd.get_args() {
                write!(f, " ")?;
                write!(f, "{arg}", arg = DisplayOsStr(arg))?;
            }
        }

        Ok(())
    }
}

struct DisplayOsStr<'a>(&'a OsStr);

impl fmt::Display for DisplayOsStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0;
        match s.to_str() {
            Some(s) => f.write_str(s),
            None => write!(f, "{s:?}"),
        }
    }
}
