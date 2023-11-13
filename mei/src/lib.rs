mod artifact;
mod fs;
mod spawn;
mod tool;
mod vars;

mod cargo {
    mod manifest;
    mod opt_level;
    mod proc;
    mod profile;
    mod target;

    pub use self::{
        manifest::Manifest,
        opt_level::OptLevel,
        proc::{cargo, Cargo},
        profile::Profile,
        target::Target,
    };
}

pub use crate::{
    artifact::{artifact, Artifact},
    cargo::{cargo, Cargo, Manifest, OptLevel, Profile, Target},
    fs::{copy, create_dir, read_to_string, write},
    spawn::Spawn,
    tool::{tool, Tool},
    vars::subdir,
};
