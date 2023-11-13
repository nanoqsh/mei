use std::path::PathBuf;

pub struct Manifest(pub(crate) PathBuf);

impl<S> From<S> for Manifest
where
    S: Into<PathBuf>,
{
    fn from(path: S) -> Self {
        let mut path = path.into();
        if path.is_dir() {
            path.push("Cargo.toml");
        }

        Self(path)
    }
}
