use {
    crate::{
        artifact::Artifact,
        cargo::{Manifest, Profile, Target},
        mei::Mei,
        spawn::{self, Info, Spawn},
    },
    std::{borrow::Cow, path::PathBuf},
};

pub fn cargo() -> Cargo {
    Cargo::default()
}

#[must_use]
#[derive(Default)]
pub struct Cargo {
    profile: Profile,
    target: Option<Target>,
    manifest: Option<Manifest>,
}

impl Cargo {
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

    pub fn manifest<S>(&mut self, manifest: S) -> &mut Self
    where
        S: Into<Manifest>,
    {
        self.manifest = Some(manifest.into());
        self
    }

    pub fn path_of(&self, artifact: &Artifact) -> PathBuf {
        let mut path = Mei::get().vars().make_mei_dir().to_owned();
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

        let mut name = Cow::default();
        if let Some(manifest) = &self.manifest {
            cargo.arg("--manifest-path").arg(manifest.path());
            name = manifest.to_str();
        }

        let mei = Mei::get();
        let target_dir = mei.vars().make_mei_dir();
        let stderr = match mei.log().stdio() {
            Ok(log) => log,
            Err(err) => panic!("failed to pipe stderr: {err}"),
        };

        cargo
            .arg("--target-dir")
            .arg(target_dir)
            .stdout(Stdio::piped())
            .stderr(stderr);

        spawn::spawn_process(
            &mut cargo,
            Info::Building {
                name: name.as_ref(),
            },
        );
    }
}
