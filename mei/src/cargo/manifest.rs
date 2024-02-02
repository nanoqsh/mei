use std::path::PathBuf;

/// The cargo manifest path.
pub struct Manifest(Box<str>);

impl Manifest {
    /// Returns a path of the manifest.
    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::from(self.as_str());
        path.push("Cargo.toml");
        path
    }

    /// Returns a path as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0[..]
    }
}

impl<S> From<S> for Manifest
where
    S: Into<Box<str>>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}
