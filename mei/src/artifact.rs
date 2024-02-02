/// Creates a new [artifact](Artifact).
pub fn artifact<S>(name: S) -> Artifact
where
    S: Into<Box<str>>,
{
    Artifact { name: name.into() }
}

/// The compilation artifact.
pub struct Artifact {
    name: Box<str>,
}

impl Artifact {
    /// Returns the name of the artifact
    /// that was given to [`artifact`] function.
    pub fn name(&self) -> &str {
        &self.name
    }
}
