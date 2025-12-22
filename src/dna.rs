use serde::{Deserialize, Serialize};
use typenum::{U0, U1, U2, Unsigned};

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

/// Aisle 0: System Lifecycle (The first pulse).
pub struct Booted;
impl Reify for Booted {
    type Id = AisleLifecycle;
    type Value = ();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Utterance;
impl Reify for Utterance {
    type Id = AisleUtterance;
    type Value = String;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stimulus;
impl Reify for Stimulus {
    type Id = AisleStimulus;
    type Value = String;
}
