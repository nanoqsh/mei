#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Profile(pub &'static str);

impl Profile {
    pub const DEV: Self = Self("dev");
    pub const TEST: Self = Self("test");
    pub const BENCH: Self = Self("bench");
    pub const RELEASE: Self = Self("release");
}

impl Default for Profile {
    fn default() -> Self {
        Self::DEV
    }
}

impl From<&'static str> for Profile {
    fn from(s: &'static str) -> Self {
        Self(s)
    }
}

impl PartialEq<str> for Profile {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<S> AsRef<S> for Profile
where
    str: AsRef<S>,
    S: ?Sized,
{
    fn as_ref(&self) -> &S {
        self.0.as_ref()
    }
}
