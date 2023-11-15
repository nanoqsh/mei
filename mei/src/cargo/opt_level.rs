#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptLevel {
    N0,
    N1,
    N2,
    N3,
    S,
    Z,
}

impl OptLevel {
    pub fn is_optimized() -> bool {
        Self::current() > Self::N0
    }

    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(Self::N0),
            "1" => Some(Self::N1),
            "2" => Some(Self::N2),
            "3" => Some(Self::N3),
            "s" => Some(Self::S),
            "z" => Some(Self::Z),
            _ => None,
        }
    }
}
