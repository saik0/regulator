// Regulator (control-systems framing):
// - facts: truth by definition (Git-derived algebra)
// - sensors: observe only
// - controller: decides/schedules
// - effectors: act only
// - factory: the physical world state we care about

pub mod controller;
pub mod dna;
pub mod effectors;
pub mod factory;
pub mod facts;
pub mod sensors;

use bevy_app::{App, ScheduleRunnerPlugin, Update};
use std::time::{SystemTime, UNIX_EPOCH};

// Unified re-exports.
pub use controller::*;
pub use dna::*;
pub use effectors::*;
pub use factory::*;
pub use facts::*;
pub use sensors::*;

use crate::effectors::checkout_effector::CheckoutEffectorPlugin;
use crate::facts::log::FactLog;

/// Internal helper for high-precision timestamps.
///
/// # Panics
///
/// This function will panic if the system clock is set to a time before
/// the Unix Epoch (January 1, 1970).
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn current_time_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time moved backwards")
        .as_nanos() as u64
}

fn startup_announcement() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        println!("_The system is holding now_");
    });
}

/// Entry point for the runtime loop.
pub fn run() {
    let mut app = App::new();

    // 1. Initialize the Chronicle
    let mut fact_log = FactLog::new();

    // 2. The First Pulse
    let boot_fact = FactEnvelope::<Utterance> {
        entity: EntityId::new("regulator-core"),
        value: "Regulator System Pulse: Online".to_string(),
        tx_time: current_time_ns(),
        valid_start: current_time_ns(),
        valid_end: None,
        op: OpKind::Assertion,
    };

    fact_log.append_envelope(boot_fact);

    // 3. Assemble the Metabolism
    app.add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(CheckoutEffectorPlugin)
        .add_plugins(ControllerPlugin) // Uses the unified plugin from systems.rs
        .insert_resource(fact_log)
        .insert_resource(TaskGraph::new(DagId::new("main-hierarchy")))
        .add_systems(Update, startup_announcement);

    println!("--- Regulator Heart: Beating ---");
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkout_effector::CheckoutEffector;
    use crate::dna::Utterance;
    use crate::facts::envelope::{FactEnvelope, OpKind};
    use crate::facts::ids::EntityId;
    use bevy_app::App;

    #[test]
    fn test_fact_propagation_to_effector() {
        let mut app = App::new();

        // 1. Setup the manifold components
        let mut fact_log = FactLog::new();
        let target_id = EntityId::new("checkout-001");

        // 2. Inject a "Start Checkout" fact into the Chronicle
        let start_fact = FactEnvelope::<Utterance> {
            entity: target_id,
            value: "Start Checkout".to_string(),
            tx_time: 1000,
            valid_start: 1000,
            valid_end: None,
            op: OpKind::Assertion,
        };
        fact_log.append_envelope(start_fact);

        // 3. Register our systems and resources
        app.insert_resource(fact_log)
            .insert_resource(CheckoutEffector::new(target_id))
            .add_systems(Update, crate::controller::systems::drive_regulator);

        // 4. Pulse the metabolism (One frame/tick)
        app.update();

        // 5. Verify the Effector "witnessed" the truth
        let effector = app.world().get_resource::<CheckoutEffector>().unwrap();

        assert_eq!(effector.last_offset().0, 1);
        // Note: Our effector updates status string based on Utterance
        // Verify it matches the logic in CheckoutEffector::observe_utterance
    }
}
