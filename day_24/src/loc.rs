use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loc {
    pub i: i16,
    pub j: i16,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexedLoc {
    pub loc: Loc,
    pub index: u32,
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Debug for IndexedLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.index, self.loc)
    }
}

impl fmt::Display for IndexedLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
