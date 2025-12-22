use crate::dna::Reify;
use crate::facts::ids::EntityId; // Ensure path matches your project
use serde::{Deserialize, Serialize};

/// The `FactEnvelope` is the universal carrier for truths in the void.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactEnvelope<A: Reify> {
    pub entity: EntityId,
    pub value: A::Value,
    pub tx_time: u64,
    pub valid_start: u64,
    pub valid_end: Option<u64>,
    pub op: OpKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpKind {
    Assertion,
    Retraction,
}

impl<A: Reify> FactEnvelope<A> {
    pub fn attribute_id(&self) -> u64 {
        A::id()
    }

    /// Algebraic check: Is this fact "true" at world-time 't'?
    pub fn is_valid_at(&self, t: u64) -> bool {
        // LAW: Start time is inclusive, end time is exclusive
        t >= self.valid_start && self.valid_end.is_none_or(|end| t < end)
    }

    /// Bridges the typed world to the storage world.
    pub fn into_raw(self) -> RawEnvelope
    where
        A::Value: Serialize,
    {
        RawEnvelope {
            entity: self.entity,
            aisle_id: A::id(),
            // We use JSON for the 'Void Edition' for human readability in logs,
            // but Phase 04 could swap this for Bincode.
            value_blob: serde_json::to_vec(&self.value).unwrap_or_default(),
            tx_time: self.tx_time,
            valid_start: self.valid_start,
            valid_end: self.valid_end,
            op: self.op,
        }
    }
}

/// The type-erased version of a Fact, suitable for storage and log-scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEnvelope {
    pub entity: EntityId,
    pub aisle_id: u64,
    pub value_blob: Vec<u8>,
    pub tx_time: u64,
    pub valid_start: u64,
    pub valid_end: Option<u64>,
    pub op: OpKind,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dna::Utterance;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_fact_validity_bounds(
            start in 0..u64::MAX / 2,
            duration in 0..1_000_000_u64,
            check_offset in 0..2_000_000_u64
        ) {
            let end = start + duration;
            let envelope = FactEnvelope::<Utterance> {
                entity: EntityId::new("test-subject"),
                value: "test".to_string(),
                tx_time: start,
                valid_start: start,
                valid_end: Some(end),
                op: OpKind::Assertion,
            };

            let check_time = start + check_offset;
            let is_valid = envelope.is_valid_at(check_time);

            if check_time >= start && check_time < end {
                prop_assert!(is_valid, "Should be valid at {}", check_time);
            } else {
                prop_assert!(!is_valid, "Should be invalid at {}", check_time);
            }
        }
    }
}
