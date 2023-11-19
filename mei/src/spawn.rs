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
                    Err(err) => {
                        let name = self.get_program().to_string_lossy();
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
                let name = self.get_program().to_string_lossy();
                panic!("failed to spawn {name} process: {err}");
            }
        }
    }
}

struct FormatProc<'a> {
    cmd: &'a Command,
    verbose: bool,
}

impl fmt::Display for FormatProc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proc = self.cmd.get_program().to_string_lossy();
        write!(f, "{proc}")?;

        if self.verbose {
            for arg in self.cmd.get_args().map(OsStr::to_string_lossy) {
                write!(f, " {arg}")?;
            }
        }

        Ok(())
    }
}
