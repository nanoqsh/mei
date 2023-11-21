use std::{
    borrow::Cow,
    error, fmt,
    path::{Path, PathBuf},
};

const TOML: &str = "Cargo.toml";

pub struct Manifest(PathBuf);

impl Manifest {
    fn new(mut path: PathBuf) -> Result<Self, IncorrectManifest> {
        if path.ends_with(TOML) {
            Ok(Self(path))
        } else if path.is_dir() {
            path.push(TOML);
            Ok(Self(path))
        } else {
            Err(IncorrectManifest)
        }
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn to_str(&self) -> Cow<str> {
        self.0
            .parent()
            .map(Path::to_string_lossy)
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct IncorrectManifest;

impl fmt::Display for IncorrectManifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the manifest must be a path to a {TOML} file")
    }
}

impl error::Error for IncorrectManifest {}

impl TryFrom<PathBuf> for Manifest {
    type Error = IncorrectManifest;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}

impl TryFrom<&str> for Manifest {
    type Error = IncorrectManifest;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::new(path.into())
    }
}

impl TryFrom<String> for Manifest {
    type Error = IncorrectManifest;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Self::new(path.into())
    }
}
