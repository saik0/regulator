use crate::dna::{Stimulus, Utterance};
use crate::effectors::Effector;
use crate::facts::envelope::FactEnvelope;
use crate::facts::ids::EntityId;
use crate::facts::offset::Offset;
use bevy_app::{App, Plugin};
use bevy_ecs::resource::Resource;

#[derive(Resource)]
pub struct CheckoutEffector {
    target_entity: EntityId,
    last_seen_offset: Offset,
    current_status: String,
}

impl CheckoutEffector {
    #[must_use]
    pub fn new(target: EntityId) -> Self {
        Self {
            target_entity: target,
            last_seen_offset: Offset::ZERO,
            current_status: "Waiting".to_string(),
        }
    }
}

impl Effector for CheckoutEffector {
    fn name(&self) -> &'static str {
        "CheckoutEffector"
    }

    fn last_offset(&self) -> Offset {
        self.last_seen_offset
    }

    fn observe_utterance(&mut self, envelope: &FactEnvelope<Utterance>, offset: Offset) {
        if envelope.entity == self.target_entity {
            self.current_status = format!("Processing: {}", envelope.value);
            self.last_seen_offset = offset;
        }
    }

    fn observe_stimulus(&mut self, envelope: &FactEnvelope<Stimulus>, offset: Offset) {
        if envelope.entity == self.target_entity {
            // Fix: Compare against String, not float
            if envelope.value == "1.0" || envelope.value == "success" {
                self.current_status = "Completed".to_string();
            }
            self.last_seen_offset = offset;
        }
    }
}

pub struct CheckoutEffectorPlugin;

impl Plugin for CheckoutEffectorPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the effector for the "main-checkout" entity
        let entity = EntityId::new("main-checkout");
        let effector = CheckoutEffector::new(entity);
        app.insert_resource(effector);
    }
}
