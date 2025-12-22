use crate::dna::Reify;
use crate::facts::envelope::{FactEnvelope, RawEnvelope};
use crate::facts::offset::Offset;
use bevy_ecs::resource::Resource;
use serde::Serialize;

/// Canonical append-only log interface.
#[derive(Resource, Default)]
pub struct FactLog {
    /// THE FIX: Store the full `RawEnvelope` manifold coordinate.
    /// This allows effectors to see the 'Value' and 'Valid Time' of facts.
    history: Vec<RawEnvelope>,
}

impl FactLog {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the `RawEnvelope` and its associated `Offset` for dispatching.
    #[must_use]
    pub fn scan_from(&self, offset: Offset) -> Vec<(RawEnvelope, Offset)> {
        // A 32-bit system might truncate usize; we accept this limit (4.2 billion facts in RAM).
        #[allow(clippy::cast_possible_truncation)]
        let start_idx = offset.0 as usize;

        self.history
            .iter()
            .enumerate()
            .skip(start_idx)
            .map(|(i, env)| {
                // The offset is the 1-based index in the history
                let current_offset = Offset((i + 1) as u64);
                // We clone here because the Log owns the truth;
                // the scanner gets a projection.
                (env.clone(), current_offset)
            })
            .collect()
    }

    /// Appends a typed `FactEnvelope` to the log by squashing it into a `RawEnvelope`.
    pub fn append_envelope<A: Reify>(&mut self, envelope: FactEnvelope<A>) -> Offset
    where
        A::Value: Serialize,
    {
        // Use the bridge method we defined in envelope.rs
        let raw = envelope.into_raw();

        self.history.push(raw);

        // Return the monotonic offset (Linear History)
        Offset(self.history.len() as u64)
    }
}
