use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use git2;

use crate::factory::state::FactoryRelativity;
use crate::facts::ids::Oid;
use crate::facts::task::RequiresGit;

/// Truth source handle (Git repository path).
#[derive(Resource)]
pub struct GitFacts {
    pub path: std::path::PathBuf,
}

impl GitFacts {
    /// Opens the underlying Git repository for inspection.
    ///
    /// # Errors
    /// Returns a `git2::Error` if the path is invalid or the repository
    /// is corrupted/inaccessible.
    pub fn open(&self) -> Result<git2::Repository, git2::Error> {
        git2::Repository::open(&self.path)
    }
}

pub struct RepoSensorPlugin {
    pub repo_path: std::path::PathBuf,
}

impl Plugin for RepoSensorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GitFacts {
            path: self.repo_path.clone(),
        })
        .add_systems(Update, observe_repo_relativity);
    }
}

/// Sensor: observes target object reachability and working tree hygiene.
///
/// Updates entities marked with `RequiresGit` with their current `FactoryRelativity`:
/// - `Unreachable`: Object does not exist in the ODB.
/// - `Drift`: Object exists but the working tree is dirty.
/// - `Settled`: Object exists and the working tree is clean.
pub fn observe_repo_relativity(
    mut commands: Commands,
    facts: Res<GitFacts>,
    query: Query<(Entity, &Oid), With<RequiresGit>>,
) {
    // 2024 Edition: Flattened early-return using let...else
    let Ok(repo) = facts.open() else { return };

    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);

    let is_dirty = repo
        .statuses(Some(&mut opts))
        .map(|s| !s.is_empty())
        .unwrap_or(true);

    for (entity, oid) in query.iter() {
        // Efficiently extract the git-compatible OID
        let Ok(git_oid) = git2::Oid::from_bytes(&oid.0) else {
            continue;
        };

        let relativity = match repo.find_object(git_oid, None) {
            Err(e) if e.code() == git2::ErrorCode::NotFound => FactoryRelativity::Unreachable,
            Err(_) => continue,
            Ok(_) => {
                if is_dirty {
                    FactoryRelativity::Drift
                } else {
                    FactoryRelativity::Settled
                }
            }
        };

        commands.entity(entity).insert(relativity);
    }
}
