use std::path::PathBuf;

pub struct Manifest(String);

impl Manifest {
    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::from(&self.0);
        if path.is_dir() {
            path.push("Cargo.toml");
        }

        path
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S> From<S> for Manifest
where
    S: Into<String>,
{
    fn from(name: S) -> Self {
        Self(name.into())
    }
}
