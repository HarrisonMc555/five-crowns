use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Score(pub u32);

impl Score {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::iter::Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Score(0), std::ops::Add::add)
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::AddAssign for Score {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
    }
}
