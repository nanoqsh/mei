use {
    crate::cargo::{OptLevel, Profile},
    std::{
        path::{Path, PathBuf},
        sync::OnceLock,
    },
};

impl OptLevel {
    pub fn current() -> Self {
        Vars::get().opt_level
    }
}

impl Profile {
    pub fn current() -> Self {
        Self(&Vars::get().profile)
    }
}

pub fn subdir(name: &str) -> PathBuf {
    let vars = Vars::get();
    vars.target_dir.join(name)
}

pub(crate) struct Vars {
    opt_level: OptLevel,
    profile: String,
    target_dir: PathBuf,
    mei_dir: OnceLock<PathBuf>,
}

impl Vars {
    fn new() -> Self {
        let profile = var("PROFILE");
        let out_dir = PathBuf::from(var("OUT_DIR"));
        let Some(target_dir) = get_target_dir(&out_dir, Path::new(&profile)) else {
            panic!("failed to find target directory");
        };

        let opt_level = {
            let level = var("OPT_LEVEL");
            match OptLevel::from_str(&level) {
                Some(level) => level,
                None => panic!("unknown OPT_LEVEL value: {level}"),
            }
        };

        Self {
            opt_level,
            profile,
            target_dir,
            mei_dir: OnceLock::new(),
        }
    }

    pub fn get() -> &'static Self {
        static VARS: OnceLock<Vars> = OnceLock::new();

        VARS.get_or_init(Self::new)
    }

    pub fn make_mei_dir(&self) -> &Path {
        self.mei_dir.get_or_init(|| {
            let mei = subdir("mei");
            crate::fs::create_dir(&mei);
            mei
        })
    }
}

fn get_target_dir(out_dir: &Path, profile: &Path) -> Option<PathBuf> {
    let mut curr = out_dir;
    while let Some(parent) = curr.parent() {
        if parent.ends_with(profile) {
            return Some(parent.parent()?.to_path_buf());
        }

        curr = parent;
    }

    None
}

fn var(key: &str) -> String {
    use std::env::{self, VarError};

    match env::var(key) {
        Ok(var) => var,
        Err(VarError::NotPresent) => panic!("the {key} variable should be set"),
        Err(VarError::NotUnicode(var)) => {
            panic!("the {key} variable should be utf-8 encoded, but {var:?} is not")
        }
    }
}
