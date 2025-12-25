// The Regulator Instance: "The Machine"
//
// STRUCTURE:
// - firmware:  The static rules (formerly DNA).
// - circuit:   The flow of sparks (formerly Metabolism).
// - facts:     The storage and ledger (The Terminator).
// - controller:The decision logic (The Nerve).
// - sensors:   Input boundary (The Membrane).
// - effectors: Output projection (The Projection).
// - runtime:   The heartbeat loop (formerly Heart).

// 1. The Modules (The Parts of the Machine)
pub mod circuit;
pub mod controller;
pub mod effectors;
pub mod factory;
pub mod facts;
pub mod firmware;
#[macro_use]
pub mod hiero;
pub mod runtime;
pub mod seal;
pub mod sensors;

use bevy_app::{App, ScheduleRunnerPlugin, Update};
use std::time::{SystemTime, UNIX_EPOCH};

// Unified re-exports for the Switchboard
pub use controller::*;
pub use effectors::*;
pub use factory::*;
pub use facts::*;
pub use firmware::*;
pub use runtime::*;
pub use sensors::*;

// Specific Imports
use crate::effectors::checkout_effector::CheckoutEffectorPlugin;
use crate::facts::journal::Journal;

/// Internal helper for high-precision timestamps.
/// Used by the Circuit to timestamp sparks.
///
/// # Panics
/// Panics if the system clock is before `UNIX_EPOCH`.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn current_time_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("FATAL: Time moved backwards")
        .as_nanos() as u64
}

fn startup_announcement() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        println!("_The Machine is holding now_");
    });
}

/// Entry point for the Regulator Instance.
/// This acts as the Switchboard, wiring the Circuit to the Runtime.
pub fn run() {
    let mut app = App::new();

    // 1. Initialize the Journal (The Terminator / WAL)
    // This is the source of Process Truth.
    let mut journal = Journal::new();

    // 2. The First Spark
    let boot_fact = FactEnvelope::<Utterance> {
        entity: EntityId::new("regulator-core"),
        value: Utterance("Hello World".to_string()),
        tx_time: current_time_ns(),
        valid_start: current_time_ns(),
        valid_end: None,
        op: OpKind::Assertion,
    };

    // Commit to the Ledger (Terminates into JSON/Bytes)
    journal.record(boot_fact);

    // 3. Assemble the Machine
    app.add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(CheckoutEffectorPlugin)
        .add_plugins(ControllerPlugin) // Wires the "Nerve" (drive_regulator)
        .insert_resource(journal) // Mounts the Ledger
        .insert_resource(TaskGraph::new(DagId::new("main-hierarchy")))
        .add_systems(Update, startup_announcement);

    println!("--- Regulator Runtime: Beating ---");
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effectors::checkout_effector::CheckoutEffector;
    use crate::facts::journal::Journal;

    #[test]
    fn test_fact_propagation_to_effector() {
        let mut app = App::new();

        // 1. Setup the Journal (Live RAM)
        let mut journal = Journal::new();
        // Use a fixed string to ensure identity matches exactly in the Regulator
        let target_id = EntityId::new("checkout-001");

        // 2. Inject a Spark into the Ledger
        let start_fact = FactEnvelope::<Utterance> {
            entity: target_id.clone(),
            value: Utterance("Hello World".to_string()),
            tx_time: 1000,
            valid_start: 1000,
            valid_end: None,
            op: OpKind::Assertion,
        };

        // Record it. The Ledger should return Offset(1).
        let new_offset = journal.record(start_fact);
        assert_eq!(
            new_offset.0, 1,
            "Ledger should return offset 1 for the first fact"
        );

        // 3. Wiring
        // Ensure we use the SAME target_id instance for the effector so it listens
        app.insert_resource(journal)
            .insert_resource(CheckoutEffector::new(target_id))
            .add_systems(Update, crate::controller::systems::drive_regulator);

        // 4. Pulse the Circuit
        app.update();

        // 5. Assert
        // The Regulator should have read the Ledger, Reified the JSON, and updated the Effector.
        let effector = app
            .world()
            .get_resource::<CheckoutEffector>()
            .expect("CheckoutEffector should be registered as a resource");

        assert_eq!(
            effector.last_offset().0,
            1,
            "Effector should have moved to offset 1. The Algebra is closed."
        );
    }
}
