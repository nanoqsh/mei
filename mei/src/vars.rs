use {
    crate::cargo::OptLevel,
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

pub fn subdir(name: &str) -> PathBuf {
    let vars = Vars::get();
    vars.target_dir.join(name)
}

pub(crate) struct Vars {
    opt_level: OptLevel,
    target_dir: PathBuf,
    mei_dir: OnceLock<PathBuf>,
}

impl Vars {
    fn new() -> Self {
        let out_dir = PathBuf::from(var("OUT_DIR"));
        let Some(target_dir) = get_target_dir(&out_dir) else {
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

/// Returns the target directory.
///
/// Currently there is no direct way to get the path, so a workaround is used.
/// The problem discussion: <https://github.com/rust-lang/cargo/issues/9661>
fn get_target_dir(mut current: &Path) -> Option<PathBuf> {
    let skip_triple = var("TARGET") == var("HOST");
    let skip_parent_dirs = if skip_triple { 4 } else { 5 };

    for _ in 0..skip_parent_dirs {
        current = current.parent()?;
    }

    Some(PathBuf::from(current))
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
