use bevy_ecs::prelude::*;
use bevy_tasks::Task;
use serde::{Deserialize, Serialize};

use crate::facts::ids::Oid;

/// Factory relativity: how the plant relates to ground truth (Git).
#[derive(Component, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactoryRelativity {
    Settled,
    Drift,
    Unreachable,
}

/// Effector is currently moving the factory toward intent.
#[derive(Component)]
pub struct CheckoutInProgress(pub Task<Result<Oid, String>>);

/// Factory is aligned to this Oid (as last verified).
#[derive(Component)]
pub struct AlignedTo(pub Oid);
