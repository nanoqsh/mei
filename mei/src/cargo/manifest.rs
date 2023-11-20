use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

pub struct Manifest(PathBuf);

impl Manifest {
    fn new(mut path: PathBuf) -> Self {
        if path.is_dir() {
            path.push("Cargo.toml");
        }

        Self(path)
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

impl<S> From<S> for Manifest
where
    S: Into<PathBuf>,
{
    fn from(name: S) -> Self {
        Self::new(name.into())
    }
}
