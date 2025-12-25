use serde::{Deserialize, Serialize};
use typenum::{U0, U1, U2, Unsigned};

// --- AISLE DEFINITIONS ---
type AisleLifecycle = U0;
type AisleUtterance = U1;
type AisleStimulus = U2;

/// THE AXIOM: The Holographic Bridge.
pub trait Reify {
    type Id: Unsigned;
    type Value: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    #[must_use]
    fn id() -> u64 {
        Self::Id::to_u64()
    }
}

// --- IDENTITY & OPERATION ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpKind {
    Assertion,
    Retraction,
}

// --- REIFIED TYPES (The Firmware Specification) ---

/// Aisle 0: System Lifecycle.
pub struct Booted;
impl Reify for Booted {
    type Id = AisleLifecycle;
    type Value = ();
}

/// Aisle 1: Utterance.
/// FIXED: Now a Tuple Struct so it can hold the data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Utterance(pub String);

impl Reify for Utterance {
    type Id = AisleUtterance;
    type Value = String; // The wire format is still just a String
}

/// Aisle 2: Stimulus.
/// FIXED: Now a Tuple Struct.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stimulus(pub String);

impl Reify for Stimulus {
    type Id = AisleStimulus;
    type Value = String;
}
