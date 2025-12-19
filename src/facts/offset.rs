#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset(pub u64);

impl Offset {
    pub const ZERO: Offset = Offset(0);
    #[must_use]
    pub fn next(self) -> Offset {
        Offset(self.0 + 1)
    }
}
