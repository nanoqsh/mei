#[derive(Clone, Copy, Eq)]
pub struct Profile(pub &'static str);

impl<S> PartialEq<S> for Profile
where
    S: AsRef<str>,
{
    fn eq(&self, other: &S) -> bool {
        self.0 == other.as_ref()
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
