use std::fmt;

pub struct Score(pub u32);

impl Score {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
