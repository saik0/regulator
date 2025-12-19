use crate::facts::offset::Offset;

#[derive(Debug, Clone)]
pub struct EventHeader {
    pub offset: Offset,
    pub timestamp_ms: u64,
    pub kind: &'static str,
    pub version: u16,
}

#[derive(Debug, Clone)]
pub enum Event {
    Booted(EventHeader),
    // Domain facts go here (lossless, append-only)
}
