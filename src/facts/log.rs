use crate::facts::{event::Event, offset::Offset};

/// Canonical append-only log interface.
/// Backed by redb (implementation intentionally omitted here).
pub struct FactLog;

impl FactLog {
    #[must_use]
    pub fn open(_path: &str) -> Self {
        FactLog
    }

    pub fn append(&mut self, _event: Event) -> Offset {
        // append event, return assigned offset
        Offset::ZERO
    }

    #[must_use]
    pub fn scan_from(&self, _offset: Offset) -> Vec<Event> {
        Vec::new()
    }
}
