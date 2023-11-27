use {
    crate::{
        cargo,
        mei::Mei,
        spawn::{self, Info, Spawn},
        vars,
    },
    std::{
        borrow::Cow,
        env,
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
        // Add the `bin` directory to the PATH variable
        let bin = vars::bin_dir();
        match env::var_os("PATH") {
            Some(path) => {
                let mut paths: Vec<_> = env::split_paths(&path).collect();
                if paths.iter().all(|p| p != bin) {
                    paths.push(bin.to_owned());
                    let new = env::join_paths(paths).expect("paths should be correct");
                    env::set_var("PATH", new);
                }
            }
            None => env::set_var("PATH", bin),
        }

        // Spawn a tool process
        match spawn::spawn_process(&mut self.0, Info::Running) {
            Ok(()) => return,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                // Install the tool if it's not found
                let name = self.name();
                install(&name);
            }
            Err(err) => {
                let name = self.name();
                panic!("failed to spawn {name} process: {err}");
            }
        }

        // Spawn the process after the tool installed
        if let Err(err) = spawn::spawn_process(&mut self.0, Info::Running) {
            let name = self.name();
            panic!("failed to spawn {name} process: {err}");
        }
    }
}

fn install(name: &str) {
    let tools = Mei::get().tools();
    let Some(tool) = tools.get(name) else {
        panic!("tool {name} not found");
    };

    let mut cargo = {
        let name = tool.from_crate.as_deref().unwrap_or(name);
        let root = vars::root_dir();
        cargo::cargo_install(name, root)
    };

    if tool.from_crate.is_some() {
        cargo.bin(name);
    }

    cargo.version(&tool.version).spawn();
}
