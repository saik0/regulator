use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::facts::ids::{Oid, TaskId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Component)]
pub enum Status {
    Pending,
    Active,
    Waiting,
    Complete,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Description(pub String);

// Markers
#[derive(Component)]
pub struct CodeTask;
#[derive(Component)]
pub struct RequiresUser;
#[derive(Component)]
pub struct RequiresGit;
#[derive(Component)]
pub struct Interactive;
#[derive(Component)]
pub struct Waiting;

#[derive(Bundle)]
pub struct CodeTaskBundle {
    pub task_id: TaskId,
    pub description: Description,
    pub status: Status,
    pub oid: Oid,
    pub code_task: CodeTask,
    pub requires_user: RequiresUser,
    pub requires_git: RequiresGit,
    pub interactive: Interactive,
}

impl CodeTaskBundle {
    #[must_use]
    pub fn new(description: String, oid: Oid) -> Self {
        let task_id = TaskId::new(&format!("{}-{}", description, oid.to_hex()));
        Self {
            task_id,
            description: Description(description),
            status: Status::Pending,
            oid,
            code_task: CodeTask,
            requires_user: RequiresUser,
            requires_git: RequiresGit,
            interactive: Interactive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_composition() {
        let mut world = World::new();
        let oid = Oid([0u8; 20]);
        let bundle = CodeTaskBundle::new("Refactor".into(), oid);

        let entity = world.spawn(bundle).id();

        assert!(world.get::<TaskId>(entity).is_some());
        assert!(world.get::<CodeTask>(entity).is_some());
        assert_eq!(*world.get::<Status>(entity).unwrap(), Status::Pending);
    }
}
