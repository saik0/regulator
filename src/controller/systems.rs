use crate::dna::{Stimulus, Utterance};
use crate::effectors::Effector;
use crate::facts::envelope::FactEnvelope;
use crate::facts::log::FactLog;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::change_detection::{Res, ResMut};

/// Regulator spine: fans out facts to effectors by offset.
/// This struct is for the "pure" non-Bevy execution path.
pub struct Regulator {
    pub log: FactLog,
    pub effectors: Vec<Box<dyn Effector>>,
}

impl Regulator {
    pub fn tick(&mut self) {
        for effector in &mut self.effectors {
            let from = effector.last_offset();
            let entries = self.log.scan_from(from);

            for (raw, offset) in entries {
                // INTERNAL REIFICATION LOGIC
                match raw.aisle_id {
                    100 => {
                        if let Ok(value) = serde_json::from_slice::<String>(&raw.value_blob) {
                            let env = FactEnvelope::<Utterance> {
                                entity: raw.entity,
                                value,
                                tx_time: raw.tx_time,
                                valid_start: raw.valid_start,
                                valid_end: raw.valid_end,
                                op: raw.op,
                            };
                            effector.observe_utterance(&env, offset);
                        }
                    }
                    200 => {
                        if let Ok(value) = serde_json::from_slice::<String>(&raw.value_blob) {
                            let env = FactEnvelope::<Stimulus> {
                                entity: raw.entity,
                                value,
                                tx_time: raw.tx_time,
                                valid_start: raw.valid_start,
                                valid_end: raw.valid_end,
                                op: raw.op,
                            };
                            effector.observe_stimulus(&env, offset);
                        }
                    }
                    _ => {} // Ignore unknown DNA aisles
                }
            }
        }
    }
}

/// Bevy wiring. This system drives the specific `CheckoutEffector` resource.
pub fn drive_regulator(
    log: Res<FactLog>,
    mut checkout: ResMut<crate::effectors::checkout_effector::CheckoutEffector>,
) {
    let from = checkout.last_offset();
    let entries = log.scan_from(from);

    for (raw, offset) in entries {
        match raw.aisle_id {
            100 => {
                if let Ok(value) = serde_json::from_slice::<String>(&raw.value_blob) {
                    let env = FactEnvelope::<Utterance> {
                        entity: raw.entity,
                        value,
                        tx_time: raw.tx_time,
                        valid_start: raw.valid_start,
                        valid_end: raw.valid_end,
                        op: raw.op,
                    };
                    checkout.observe_utterance(&env, offset);
                }
            }
            200 => {
                if let Ok(value) = serde_json::from_slice::<String>(&raw.value_blob) {
                    let env = FactEnvelope::<Stimulus> {
                        entity: raw.entity,
                        value,
                        tx_time: raw.tx_time,
                        valid_start: raw.valid_start,
                        valid_end: raw.valid_end,
                        op: raw.op,
                    };
                    checkout.observe_stimulus(&env, offset);
                }
            }
            _ => {}
        }
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        // The heart of the metabolism: pulse the regulator every frame
        app.add_systems(Update, drive_regulator);
    }
}
