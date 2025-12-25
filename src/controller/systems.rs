use crate::effectors::Effector;
use crate::facts::envelope::FactEnvelope;
use crate::facts::journal::Journal;
use crate::firmware::{Stimulus, Utterance};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::change_detection::{Res, ResMut};

/// Regulator spine: fans out facts to effectors by offset.
pub struct Regulator {
    pub journal: Journal,
    pub effectors: Vec<Box<dyn Effector>>,
}

impl Regulator {
    pub fn tick(&mut self) {
        for effector in &mut self.effectors {
            let from = effector.last_offset();
            let entries = self.journal.scan_from(from);

            for (raw, offset) in entries {
                match raw.aisle_id {
                    1 => {
                        // AisleUtterance (U1)
                        if let Ok(value_str) = serde_json::from_slice::<String>(&raw.value_blob) {
                            let env = FactEnvelope::<Utterance> {
                                entity: raw.entity,
                                // WRAP: String -> Utterance(String)
                                value: Utterance(value_str),
                                tx_time: raw.tx_time,
                                valid_start: raw.valid_start,
                                valid_end: raw.valid_end,
                                op: raw.op,
                            };
                            effector.observe_utterance(&env, offset);
                        } else {
                            // LOGGING FIX: Use {:?} for Offset
                            eprintln!("CRITICAL: Witness failed for Aisle 1 at offset {offset:?}");
                        }
                    }
                    2 => {
                        // AisleStimulus (U2)
                        if let Ok(value_str) = serde_json::from_slice::<String>(&raw.value_blob) {
                            let env = FactEnvelope::<Stimulus> {
                                entity: raw.entity,
                                // WRAP: String -> Stimulus(String)
                                value: Stimulus(value_str),
                                tx_time: raw.tx_time,
                                valid_start: raw.valid_start,
                                valid_end: raw.valid_end,
                                op: raw.op,
                            };
                            effector.observe_stimulus(&env, offset);
                        } else {
                            eprintln!("CRITICAL: Witness failed for Aisle 2 at offset {offset:?}");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Bevy system: Drives the regulator logic for every frame.
///
/// # Panics
/// Panics if the Journal contains corrupted JSON data.
/// This is a "Chain of Custody" violation and requires an immediate crash.
pub fn drive_regulator(
    journal: Res<Journal>,
    mut checkout: ResMut<crate::effectors::checkout_effector::CheckoutEffector>,
) {
    let from = checkout.last_offset();
    let entries = journal.scan_from(from);

    for (raw, offset) in entries {
        match raw.aisle_id {
            1 => {
                let value_str = serde_json::from_slice::<String>(&raw.value_blob)
                    .expect("FATAL: Journal corruption. Chain of Custody broken on Aisle 1.");

                let env = FactEnvelope::<Utterance> {
                    entity: raw.entity,
                    // WRAP
                    value: Utterance(value_str),
                    tx_time: raw.tx_time,
                    valid_start: raw.valid_start,
                    valid_end: raw.valid_end,
                    op: raw.op,
                };
                checkout.observe_utterance(&env, offset);
            }
            2 => {
                let value_str = serde_json::from_slice::<String>(&raw.value_blob)
                    .expect("FATAL: Journal corruption. Chain of Custody broken on Aisle 2.");

                let env = FactEnvelope::<Stimulus> {
                    entity: raw.entity,
                    // WRAP
                    value: Stimulus(value_str),
                    tx_time: raw.tx_time,
                    valid_start: raw.valid_start,
                    valid_end: raw.valid_end,
                    op: raw.op,
                };
                checkout.observe_stimulus(&env, offset);
            }
            _ => {}
        }
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drive_regulator);
    }
}
