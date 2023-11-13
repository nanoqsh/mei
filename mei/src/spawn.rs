use std::process::Command;

pub trait Spawn {
    fn spawn(&mut self);
}

impl Spawn for Command {
    fn spawn(&mut self) {
        match self.spawn() {
            Ok(child) => {
                let out = match child.wait_with_output() {
                    Ok(out) => out,
                    Err(err) => panic!(
                        "failed to wait the output from {name:?} process: {err}",
                        name = self.get_program(),
                    ),
                };

                // TODO: log
                println!("cargo:warning={out:?}");

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
