// Regulator (control-systems framing):
// - facts: truth by definition (Git-derived algebra)
// - sensors: observe only
// - controller: decides/schedules
// - effectors: act only
// - factory: the physical world state we care about

pub mod controller;
pub mod effectors;
pub mod factory;
pub mod facts;
pub mod sensors;

// Public convenience re-exports.
// If this grows noisy, replace it with a `prelude` module.
pub use controller::*;
pub use effectors::*;
pub use factory::*;
pub use facts::*;
pub use sensors::*;

/// Entry point for the runtime loop.
/// Call from a binary as: `regulator::run()`.
///
/// # Panics
///
/// Panics if the current working directory cannot be accessed or determined,
/// which is required to initialize the sensor plugins.
#[cfg(not(test))]
pub fn run() {
    use bevy_app::{App, ScheduleRunnerPlugin, TaskPoolPlugin, Update};

    use crate::controller::ControllerPlugin;
    use crate::effectors::checkout_effector::CheckoutEffectorPlugin;
    use crate::facts::{DagId, TaskGraph};
    use crate::sensors::{
        fs_sensor::FsSensorPlugin,
        repo_sensor::{RepoSensorPlugin, observe_repo_relativity},
    };

    println!("--- Regulator Starting --- ");

    let repo_path = std::env::current_dir().expect("current_dir");

    App::new()
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(TaskPoolPlugin::default())
        .insert_resource(TaskGraph::new(DagId::new("main-history")))
        .add_plugins(FsSensorPlugin {
            repo_path: repo_path.clone(),
        })
        .add_plugins(RepoSensorPlugin {
            repo_path: repo_path.clone(),
        })
        .add_plugins(ControllerPlugin)
        .add_plugins(CheckoutEffectorPlugin)
        .add_systems(Update, observe_repo_relativity)
        .run();
}

#[cfg(not(test))]
#[cfg(test)] // This prevents the test run from conflicting with the main run definition
pub fn run() {
    // no-op: tests should not spin the app loop
}
