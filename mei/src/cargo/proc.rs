use {
    crate::{
        artifact::Artifact,
        cargo::{Manifest, Profile, Target},
        mei::Mei,
        spawn::{self, Info, Spawn},
        var,
    },
    semver::VersionReq,
    std::{
        path::Path,
        path::PathBuf,
        process::{Command, Stdio},
    },
};

/// Creates a new [cargo](Cargo) object.
pub fn cargo_build() -> Cargo<Build> {
    Cargo {
        profile: Profile::DEV,
        target: None,
        mode: Build { manifest: None },
    }
}

pub(crate) fn cargo_install<'a>(name: &'a str, root: &'a Path) -> Cargo<Install<'a>> {
    Cargo {
        profile: Profile::RELEASE,
        target: None,
        mode: Install {
            name,
            root,
            bin: None,
            version: None,
        },
    }
}

/// The object of a cargo build process.
#[must_use]
pub struct Cargo<M> {
    profile: Profile,
    target: Option<Target>,
    mode: M,
}

impl<M> Cargo<M> {
    pub fn profile<P>(&mut self, profile: P) -> &mut Self
    where
        P: Into<Profile>,
    {
        self.profile = profile.into();
        self
    }

    pub fn target(&mut self, target: Target) -> &mut Self {
        self.target = Some(target);
        self
    }

    pub fn path_of(&self, artifact: &Artifact) -> PathBuf {
        let mut path = var::mei_dir().to_owned();
        if let Some(Target(target)) = self.target {
            path.push(target);
        }

        path.push(self.profile.target_dir_name());
        path.push(artifact.name());
        path
    }
}

fn spawn<M>(proc: &mut Cargo<M>)
where
    M: Mode,
{
    let mut cargo = Command::new("cargo");
    let info = proc.mode.mode(&mut cargo, proc.profile);
    if let Some(Target(target)) = proc.target {
        cargo.args(["--target", target]);
    }

    let stderr = match Mei::get().log().stdio() {
        Ok(log) => log,
        Err(err) => panic!("failed to pipe stderr: {err}"),
    };

    cargo
        .arg("--target-dir")
        .arg(var::mei_dir())
        .stdout(Stdio::piped())
        .stderr(stderr);

    if let Err(err) = spawn::spawn_process(&mut cargo, info) {
        panic!("failed to spawn cargo process: {err}");
    }
}

impl<M> Spawn for Cargo<M>
where
    M: Mode,
{
    fn spawn(&mut self) {
        spawn(self);
    }
}

trait Mode {
    fn mode(&self, cmd: &mut Command, profile: Profile) -> Info;
}

pub struct Build {
    manifest: Option<Manifest>,
}

impl Cargo<Build> {
    pub fn manifest<S>(&mut self, manifest: S) -> &mut Self
    where
        S: Into<Manifest>,
    {
        self.mode.manifest = Some(manifest.into());
        self
    }

    /// Spawns the cargo process.
    ///
    /// This is a shortcut method of the [`Spawn`] trait
    /// without having to import this in the scope.
    pub fn spawn(&mut self) {
        spawn(self);
    }
}

impl Mode for Build {
    fn mode(&self, cmd: &mut Command, profile: Profile) -> Info {
        cmd.arg("build");

        match profile {
            Profile::DEV => {} // default build profile
            Profile::RELEASE => _ = cmd.arg("--release"),
            Profile(profile) => _ = cmd.args(["--profile", profile]),
        }

        Info::Building {
            name: match &self.manifest {
                Some(m) => {
                    cmd.arg("--manifest-path").arg(m.path());
                    m.as_str()
                }
                None => "",
            },
        }
    }
}

pub(crate) struct Install<'a> {
    name: &'a str,
    root: &'a Path,
    bin: Option<&'a str>,
    version: Option<&'a VersionReq>,
}

impl<'a> Cargo<Install<'a>> {
    pub(crate) fn bin(&mut self, bin: &'a str) -> &mut Self {
        self.mode.bin = Some(bin);
        self
    }

    pub(crate) fn version(&mut self, version: &'a VersionReq) -> &mut Self {
        self.mode.version = Some(version);
        self
    }
}

impl Mode for Install<'_> {
    fn mode(&self, cmd: &mut Command, profile: Profile) -> Info {
        cmd.args(["install", self.name])
            .arg("--root")
            .arg(self.root);

        match profile {
            Profile::RELEASE => {} // default install profile
            Profile::DEV => _ = cmd.arg("--debug"),
            Profile(profile) => _ = cmd.args(["--profile", profile]),
        }

        if let Some(bin) = self.bin {
            cmd.args(["--bin", bin]);
        }

        if let Some(version) = self.version {
            let version = version.to_string();
            cmd.arg("--version").arg(version);
        }

        Info::Installing { name: self.name }
    }
}
