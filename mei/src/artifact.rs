pub fn artifact<S>(name: S) -> Artifact
where
    S: Into<Box<str>>,
{
    Artifact::new(name.into())
}

pub struct Artifact {
    name: Box<str>,
}

impl Artifact {
    fn new(name: Box<str>) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
