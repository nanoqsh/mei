use {
    crate::{cargo::OptLevel, env, mei::Mei},
    std::{
        path::{Path, PathBuf},
        sync::OnceLock,
    },
};

impl OptLevel {
    /// Returns the current optimization level.
    pub fn current() -> Self {
        Mei::get().vars().opt_level
    }
}

/// Returns the target directory path.
pub fn target_dir() -> &'static Path {
    &Mei::get().vars().target_dir
}

/// Returns the root directory path.
pub fn root_dir() -> &'static Path {
    &Mei::get().vars().root_dir
}

/// Returns the bin directory path.
pub fn bin_dir() -> &'static Path {
    &Mei::get().vars().bin_dir
}

/// Returns the mei working directory path.
pub fn mei_dir() -> &'static Path {
    Mei::get().vars().make_mei_dir()
}

pub(crate) struct Vars {
    opt_level: OptLevel,
    root_dir: PathBuf,
    bin_dir: PathBuf,
    target_dir: PathBuf,
    mei_dir: OnceLock<PathBuf>,
}

impl Vars {
    pub fn new() -> Self {
        let opt_level = {
            let level = env::var("OPT_LEVEL");
            match OptLevel::from_str(&level) {
                Some(level) => level,
                None => panic!("unknown OPT_LEVEL value: {level}"),
            }
        };

        let out_dir = PathBuf::from(env::var("OUT_DIR"));
        let Some(target_dir) = get_target_dir(&out_dir) else {
            panic!("failed to find target directory");
        };

        let root_dir = match target_dir.parent() {
            Some(root_dir) => root_dir.to_owned(),
            None => panic!("failed to find root directory"),
        };

        let bin_dir = root_dir.join("bin");
        Self {
            opt_level,
            target_dir,
            root_dir,
            bin_dir,
            mei_dir: OnceLock::new(),
        }
    }

    pub fn make_mei_dir(&self) -> &Path {
        self.mei_dir.get_or_init(|| {
            let mei = self.target_dir.join("mei");
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
    let skip_triple = env::var("TARGET") == env::var("HOST");
    let skip_parent_dirs = if skip_triple { 4 } else { 5 };

    for _ in 0..skip_parent_dirs {
        current = current.parent()?;
    }

    Some(PathBuf::from(current))
}
