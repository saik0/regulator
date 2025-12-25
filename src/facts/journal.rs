use crate::facts::envelope::{FactEnvelope, RawEnvelope};
use crate::firmware::Reify;
use crate::offset::Offset;
use bevy_ecs::prelude::Resource;
use serde::Serialize;

/// The Journal is the canonical "Write-Ahead Log" (WAL) for the Machine.
///
/// ALGEBRA:
/// This is the "Process Truth." It converts typed sparks into raw bytes.
/// Any system can reconstruct its state by scanning this sequence from 0.
#[derive(Resource, Default)]
pub struct Journal {
    /// The chronological record of all valid sparks.
    history: Vec<RawEnvelope>,
}

impl Journal {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// THE TERMINATOR: Commits a typed spark to the immutable record.
    ///
    /// WHY: This is the exact point where "Thinking" becomes "Fact."
    /// We use JSON serialization to ensure the bytes are queryable
    /// by the Regulator later.
    pub fn record<A>(&mut self, envelope: FactEnvelope<A>) -> Offset
    where
        A: Reify + Serialize,
    {
        // 1. Squash the typed fact into the Raw protocol bytes.
        let raw = envelope.into_raw();

        // 2. Commit to the in-memory history.
        self.history.push(raw);

        // 3. Return the 1-based offset (Sequence Number).
        Offset(self.history.len() as u64)
    }

    /// The Scanner: Used by the Regulator (Nerve) to catch up.
    #[must_use]
    pub fn scan_from(&self, offset: Offset) -> Vec<(RawEnvelope, Offset)> {
        #[allow(clippy::cast_possible_truncation)]
        let start_idx = offset.0 as usize;

        self.history
            .iter()
            .enumerate()
            .skip(start_idx)
            .map(|(i, env)| {
                // Return the record and its 1-based sequence number.
                let current_offset = Offset((i + 1) as u64);
                (env.clone(), current_offset)
            })
            .collect()
    }

    /// Observability: Returns the heap footprint of the Journal.
    #[must_use]
    pub fn memory_usage_bytes(&self) -> usize {
        self.history.len() * std::mem::size_of::<RawEnvelope>()
            + self
                .history
                .iter()
                .map(|e| e.value_blob.len())
                .sum::<usize>()
    }
}
