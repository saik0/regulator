use crate::EntityId;
use crate::firmware::{OpKind, Reify};
use serde::{Deserialize, Serialize};

/// The Wire Format (The "Journal Entry").
///
/// This is 'pub' because it acts as the universal protocol for the Circuit.
/// The Journal stores it, and the Regulator inspects it to determine Aisle IDs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawEnvelope {
    pub entity: EntityId,
    pub aisle_id: u64,
    pub value_blob: Vec<u8>,
    pub tx_time: u64,
    pub valid_start: u64,
    pub valid_end: Option<u64>,
    pub op: OpKind,
}

/// The Typed Carrier (The "Metabolic Spark").
///
/// Used by Sensors to emit facts and by the Regulator to digest them.
#[derive(Debug, Clone)]
pub struct FactEnvelope<A> {
    pub entity: EntityId,
    pub value: A,
    pub tx_time: u64,
    pub valid_start: u64,
    pub valid_end: Option<u64>,
    pub op: OpKind,
}

impl<A: Reify> FactEnvelope<A> {
    /// Converts the typed envelope into a raw Journal Entry.
    ///
    /// # Panics
    /// Panics if `serde_json` fails to serialize the inner value.
    /// Since `A` is constrained to `Serialize`, this implies a catastrophic
    /// memory failure or invalid structural state.
    pub fn into_raw(self) -> RawEnvelope
    where
        A: Serialize,
    {
        RawEnvelope {
            entity: self.entity,
            aisle_id: A::id(),
            // THE BRIDGE: Symmetrical JSON serialization ensures the Regulator
            // can reify this later.
            value_blob: serde_json::to_vec(&self.value)
                .expect("FATAL: Firmware serialization failure during WAL termination."),
            tx_time: self.tx_time,
            valid_start: self.valid_start,
            valid_end: self.valid_end,
            op: self.op,
        }
    }
}
