use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use uuid::{Uuid, uuid};

/// The "Magic" Namespace for deterministic ID generation.
///
/// We use a fixed `UUIDv5` namespace to ensure that identity is derived
/// solely from content/name. This enables:
/// - Idempotency: The same task name always yields the same ID across runs.
/// - Merging: Multiple sensors can identify the same entity without a central registry.
/// - Integrity: The ID acts as a cryptographic fingerprint of the task's intent.
#[allow(dead_code)] // Declared here for documentation purposes only.
const REGULATOR_NAMESPACE: Uuid = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");

/// Domain-specific namespaces (v5-derived from `REGULATOR_NAMESPACE` + "task", "dag", "entity").
pub const TASK_NAMESPACE: Uuid = uuid!("d2c94301-90a6-5743-982c-a299385501ca");
pub const DAG_NAMESPACE: Uuid = uuid!("93e5066c-5197-5264-803a-3c990263f338");
pub const ENTITY_NAMESPACE: Uuid = uuid!("54a3c390-e880-5fb5-9439-82fbe1b10c21");

/// Content-Addressable Object Identifier (Git Oid bytes).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Component,
)]
pub struct Oid(pub [u8; 20]);

impl Oid {
    /// Creates an `Oid` from a 40-character hex string.
    ///
    /// # Errors
    /// Returns an error if the string length is not 40 or contains invalid hex.
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 40 {
            return Err("Oid must be 40 hex chars".into());
        }
        let mut bytes = [0u8; 20];
        for i in 0..20 {
            bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("Invalid hex: {e}"))?;
        }
        Ok(Self(bytes))
    }

    /// Converts the identifier to a 40-character hex string.
    #[must_use]
    pub fn to_hex(&self) -> String {
        let mut s = String::with_capacity(40);
        for b in self.0 {
            let _ = write!(s, "{b:02x}");
        }
        s
    }

    /// Explicit boundary conversion to `git2::Oid` (keeps facts layer pure).
    ///
    /// # Panics
    /// Panics if the internal bytes are not a valid Git Oid.
    #[must_use]
    pub fn as_git(&self) -> git2::Oid {
        git2::Oid::from_bytes(&self.0).expect("valid git oid")
    }
}

impl From<git2::Oid> for Oid {
    fn from(oid: git2::Oid) -> Self {
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(oid.as_bytes());
        Oid(bytes)
    }
}

/// Deterministic Identity for Tasks.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize, Component,
)]
pub struct TaskId(pub Uuid);

impl TaskId {
    /// Generates a new `TaskId` derived from a name string via `UUIDv5`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(Uuid::new_v5(&TASK_NAMESPACE, name.as_bytes()))
    }
}

/// Deterministic Identity for Directed Acyclic Graphs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize, Component,
)]
pub struct DagId(pub Uuid);

impl DagId {
    /// Generates a new `DagId` derived from a name string via `UUIDv5`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(Uuid::new_v5(&DAG_NAMESPACE, name.as_bytes()))
    }
}

// src/facts/ids.rs

/// Deterministic Identity for Entities in the manifold.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize, Component,
)]
pub struct EntityId(pub Uuid);

impl EntityId {
    /// Generates a new `EntityId` derived from a name string via `UUIDv5`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(Uuid::new_v5(&ENTITY_NAMESPACE, name.as_bytes()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_oid_hex_roundtrip(bytes in any::<[u8; 20]>()) {
            let original = Oid(bytes);
            let hex = original.to_hex();
            let reconstructed = Oid::from_hex(&hex).unwrap();
            prop_assert_eq!(original, reconstructed);
        }
    }
    proptest! {
        #[test]
        fn test_task_id_is_idempotent(s in "\\PC*") {
            let id_a = TaskId::new(&s);
            let id_b = TaskId::new(&s);
            prop_assert_eq!(id_a, id_b);
        }

        #[test]
        fn test_task_id_namespace_stability(s in "\\PC*") {
            let id = TaskId::new(&s);
            // Verify it uses the Regulator Namespace specifically
            let expected = uuid::Uuid::new_v5(&TASK_NAMESPACE, s.as_bytes());
            prop_assert_eq!(id.0, expected);
        }
    }
}
