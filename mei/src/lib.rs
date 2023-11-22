mod artifact;
mod config;
mod fs;
mod log;
mod mei;
mod spawn;
mod vars;

mod cargo {
    mod manifest;
    mod opt_level;
    mod proc;
    mod profile;
    mod target;

    pub(crate) use self::proc::cargo_install;

    pub use self::{
        manifest::Manifest,
        opt_level::OptLevel,
        proc::{cargo_build, Cargo},
        profile::Profile,
        target::Target,
    };
}

mod tool {
    mod proc;
    mod tools;

    pub use self::proc::{tool, Tool};
    pub(crate) use self::tools::Tools;
}

pub use crate::{
    artifact::{artifact, Artifact},
    cargo::{cargo_build, Cargo, Manifest, OptLevel, Profile, Target},
    fs::{copy, create_dir, read_to_string, write},
    spawn::Spawn,
    tool::{tool, Tool},
    vars::{bin_dir, target_dir},
};
