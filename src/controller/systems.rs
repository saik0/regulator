use bevy_app::{App, Plugin};

use crate::effectors::Effector;
use crate::facts::log::FactLog;

/// Regulator spine: fans out facts to effectors by offset.
pub struct Regulator {
    log: FactLog,
    effectors: Vec<Box<dyn Effector>>,
}

impl Regulator {
    pub fn tick(&mut self) {
        for effector in &mut self.effectors {
            let from = effector.last_offset();
            let events = self.log.scan_from(from);
            for event in events {
                effector.apply(&event);
            }
        }
    }
}

/// Bevy wiring only. No logic.
pub struct RegulatorPlugin;
pub struct ControllerPlugin;

impl Plugin for RegulatorPlugin {
    fn build(&self, _app: &mut App) {}
}

impl Plugin for ControllerPlugin {
    fn build(&self, _app: &mut App) {}
}
