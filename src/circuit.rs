// src/circuit.rs
//
// ROLE: The Conductive Path (Guarded).
// This module provides the traits and helpers for moving Sparks (Facts)
// through the Machine's internal wiring.
//
// REGULATOR UPDATE:
// This circuit is now "Guarded". It requires facts to be 'Sealed'
// before they can be energized into the Journal.

use crate::facts::envelope::FactEnvelope;
use crate::facts::journal::Journal;
use crate::firmware::Reify;
use crate::offset::Offset;
use bevy_ecs::system::ResMut;
use serde::Serialize;

// --- THE SEAL (The Lens) ---

/// A wrapper that proves a Fact has been vetted by the Regulator.
/// You cannot construct this directly; you must use `mint` to prove intent.
///
/// This is the "Proof Token" that allows an Agent to stop reasoning and
/// simply show their badge to the circuit.
#[derive(Debug, Clone)]
pub struct Sealed<T>(T);

impl<T> Sealed<T> {
    /// The Lens Correction.
    /// This function is the ONLY way to create a Sealed<T>.
    ///
    /// # Errors
    /// Returns an error string if the `validator` closure returns `Err`.
    /// This indicates the fact violated the specified Law.
    pub fn mint<F>(fact: T, validator: F) -> Result<Self, String>
    where
        F: FnOnce(&T) -> Result<(), String>,
    {
        match validator(&fact) {
            Ok(()) => Ok(Sealed(fact)),
            Err(e) => Err(format!("REGULATOR REJECTION: {e}")),
        }
    }

    /// The Witness Accessor.
    /// Allows the Trusted Core (Systems) to unwrap the fact.
    /// This consumes the seal, as the fact is now part of the immutable record.
    pub fn witness(self) -> T {
        self.0
    }
}

// --- THE ENERGIZED PATH ---

/// The Energize trait allows a component to push a NEW spark into the Machine.
///
/// REGULATOR CHANGE: This now strictly requires `Sealed<FactEnvelope<A>>`.
/// Unverified facts are physically rejected by the type system.
pub trait Energize {
    fn record_spark<A>(&mut self, fact: Sealed<FactEnvelope<A>>) -> Offset
    where
        A: Reify + Serialize;
}

// Implement Energize for the Journal resource.
impl Energize for Journal {
    fn record_spark<A>(&mut self, fact: Sealed<FactEnvelope<A>>) -> Offset
    where
        A: Reify + Serialize,
    {
        // 1. Witness the Seal.
        // The Journal trusts the Seal because the Type System guarantees
        // it passed the `mint` gate.
        let verified_envelope = fact.witness();

        // 2. Commit to storage.
        // The Journal is the 'Terminator' where the spark becomes a record.
        self.record(verified_envelope)
    }
}

/// A helper for systems that need to emit facts directly.
///
/// This signature forces the calling Agent to have already successfully
/// run `Sealed::mint()`.
pub fn emit_fact<A>(mut journal: ResMut<Journal>, proof: Sealed<FactEnvelope<A>>) -> Offset
where
    A: Reify + Serialize,
{
    journal.record_spark(proof)
}
