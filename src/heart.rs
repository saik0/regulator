use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use crate::dna::{Reify, Utterance, Stimulus};
use crate::facts::envelope::{FactEnvelope, OpKind};
use crate::facts::log::FactLog;
use crate::facts::ids::EntityId;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Event)]
pub struct Spark<A: Reify> {
    pub entity: EntityId,
    pub value: A::Value,
}

/// The Pure Algebra of Promotion.
/// Converts a transient Spark into a Bitemporal Fact.
pub fn promote_spark<A: Reify>(spark: Spark<A>, now: u64) -> FactEnvelope<A> {
    FactEnvelope {
        entity: spark.entity,
        value: spark.value,
        tx_time: now,
        valid_start: now,
        valid_end: None,
        op: OpKind::Assertion,
    }
}

pub struct HeartPlugin;

impl Plugin for HeartPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Spark<Utterance>>()
            .add_event::<Spark<Stimulus>>()
            .add_systems(Update, (promote_utterances, promote_stimuli));
    }
}

fn promote_utterances(mut events: EventReader<Spark<Utterance>>, mut log: ResMut<FactLog>) {
    let now = current_time_ns();
    for spark in events.read() {
        let fact = promote_spark(
            Spark { entity: spark.entity, value: spark.value.clone() },
            now
        );
        log.append_envelope(fact);
    }
}

fn promote_stimuli(mut events: EventReader<Spark<Stimulus>>, mut log: ResMut<FactLog>) {
    let now = current_time_ns();
    for spark in events.read() {
        let fact = promote_spark(
            Spark { entity: spark.entity, value: spark.value.clone() },
            now
        );
        log.append_envelope(fact);
    }
}

fn current_time_ns() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_promotion_invariants(
            entity_name in "\\PC*",
            message in "\\PC*",
            timestamp in 0..u64::MAX
        ) {
            let entity = EntityId::new(&entity_name);
            let spark = Spark::<Utterance> {
                entity,
                value: message.clone(),
            };

            let fact = promote_spark(spark, timestamp);

            // INVARIANT 1: Identity preservation
            prop_assert_eq!(fact.entity, entity);

            // INVARIANT 2: Value preservation
            prop_assert_eq!(fact.value, message);

            // INVARIANT 3: Bitemporal "Now" window
            // The fact must be valid starting exactly when it was learned.
            prop_assert_eq!(fact.tx_time, timestamp);
            prop_assert_eq!(fact.valid_start, timestamp);
            prop_assert!(fact.valid_end.is_none());

            // INVARIANT 4: Promotion is always an Assertion
            prop_assert_eq!(fact.op, OpKind::Assertion);
        }
    }
}