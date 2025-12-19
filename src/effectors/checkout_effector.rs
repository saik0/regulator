// src/effectors/checkout_effector.rs
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use git2::build::CheckoutBuilder;

use crate::factory::state::{AlignedTo, CheckoutInProgress, FactoryRelativity};
use crate::facts::ids::{Oid, TaskId};
use crate::facts::task::Status;
use crate::sensors::repo_sensor::GitFacts;

/// Effector: when the controller has selected Active + Settled, physically checkout the Oid.
pub struct CheckoutEffectorPlugin;

impl Plugin for CheckoutEffectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_checkout, poll_checkout));
    }
}

// ---- local readability aliases --------------------------------------------
//
// Alias the "query parts" (Q and Filter), not Query<'w,'s,...> itself.
// Use reference forms because Bevy QueryData is `&T` / `&mut T` (not `T`).

type SpawnCheckoutItem<'w> = (Entity, &'w Oid, &'w Status, &'w FactoryRelativity);
type SpawnCheckoutFilter = (
    Changed<Status>,
    Without<CheckoutInProgress>,
    Without<AlignedTo>,
);

type PollCheckoutItem<'w> = (Entity, &'w mut CheckoutInProgress, &'w TaskId);

// ---- systems ---------------------------------------------------------------

fn spawn_checkout(
    mut commands: Commands,
    facts: Res<GitFacts>,
    query: Query<SpawnCheckoutItem<'_>, SpawnCheckoutFilter>,
) {
    let pool = AsyncComputeTaskPool::get();

    for (entity, oid, status, rel) in query.iter() {
        if matches!(status, Status::Active) && matches!(rel, FactoryRelativity::Settled) {
            let repo_path = facts.path.clone();
            let target = *oid;

            let task: Task<Result<Oid, String>> = pool.spawn(async move {
                let repo = git2::Repository::open(repo_path).map_err(|e| e.to_string())?;
                let git_oid = git2::Oid::from_bytes(&target.0).map_err(|e| e.to_string())?;
                let object = repo.find_object(git_oid, None).map_err(|e| e.to_string())?;

                let mut checkout_opts = CheckoutBuilder::new();
                checkout_opts.force();

                repo.checkout_tree(&object, Some(&mut checkout_opts))
                    .map_err(|e| e.to_string())?;
                repo.set_head_detached(git_oid).map_err(|e| e.to_string())?;

                Ok(target)
            });

            commands.entity(entity).insert(CheckoutInProgress(task));
        }
    }
}

fn poll_checkout(mut commands: Commands, mut query: Query<PollCheckoutItem<'_>>) {
    for (entity, mut in_progress, task_id) in &mut query {
        if let Some(result) = future::block_on(future::poll_once(&mut in_progress.0)) {
            match result {
                Ok(oid) => {
                    println!(
                        "Regulator: effector settled {} at {}",
                        task_id.0,
                        oid.to_hex()
                    );
                    commands
                        .entity(entity)
                        .insert(AlignedTo(oid))
                        .remove::<CheckoutInProgress>();
                }
                Err(e) => {
                    eprintln!("Regulator: effector failed: {e}");
                    commands.entity(entity).remove::<CheckoutInProgress>();
                }
            }
        }
    }
}
