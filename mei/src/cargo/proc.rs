use {
    crate::{
        artifact::Artifact,
        cargo::{Manifest, Profile, Target},
        spawn::Spawn,
        vars::Vars,
    },
    std::path::PathBuf,
};

pub fn cargo() -> Cargo {
    Cargo::default()
}

#[derive(Default)]
pub struct Cargo {
    profile: Profile,
    target: Option<Target>,
    manifest: Option<Manifest>,
}

impl Cargo {
    pub fn profile<P>(mut self, profile: P) -> Self
    where
        P: Into<Profile>,
    {
        self.profile = profile.into();
        self
    }

    pub fn target(mut self, target: Target) -> Self {
        self.target = Some(target);
        self
    }

    pub fn manifest<S>(mut self, manifest: S) -> Self
    where
        S: Into<Manifest>,
    {
        self.manifest = Some(manifest.into());
        self
    }

    pub fn path_of(&self, artifact: &Artifact) -> PathBuf {
        let vars = Vars::get();
        let mut path = vars.make_mei_dir().to_owned();
        if let Some(Target(target)) = self.target {
            path.push(target);
        }

        path.push(self.profile.target_dir_name());
        path.push(artifact.name());
        path
    }
}

impl Spawn for Cargo {
    fn spawn(&mut self) {
        use std::process::{Command, Stdio};

        let mut cargo = Command::new("cargo");
        cargo.arg("build");

        match self.profile {
            Profile::DEV => {} // default cargo profile
            Profile::RELEASE => _ = cargo.arg("--release"),
            Profile(profile) => _ = cargo.args(["--profile", profile]),
        }

        if let Some(Target(target)) = self.target {
            cargo.args(["--target", target]);
        }

        if let Some(Manifest(manifest)) = &self.manifest {
            cargo.arg("--manifest-path").arg(manifest);
        }

        let vars = Vars::get();
        let target_dir = vars.make_mei_dir();

        cargo
            .arg("--target-dir")
            .arg(&target_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Spawn::spawn(&mut cargo);
    }
}
