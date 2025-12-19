use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use notify::{Config, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::{Mutex, mpsc};

/// Sensor: file-system change hints (signals only; not truth).
#[derive(Resource)]
pub struct FsSensor {
    _watcher: notify::RecommendedWatcher,
    pub receiver: Mutex<mpsc::Receiver<notify::Result<notify::Event>>>,
}

/// A simple “something changed recently” latch.
/// Controller/effectors can use it as a hint to re-verify alignment.
#[derive(Resource, Default)]
pub struct FsChanged(pub bool);

pub struct FsSensorPlugin {
    pub repo_path: PathBuf,
}

impl Plugin for FsSensorPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::RecommendedWatcher::new(tx, Config::default())
            .expect("Failed to create watcher");
        watcher
            .watch(&self.repo_path, RecursiveMode::Recursive)
            .ok();

        app.insert_resource(FsSensor {
            _watcher: watcher,
            receiver: Mutex::new(rx),
        })
        .init_resource::<FsChanged>()
        .add_systems(Update, latch_fs_events);
    }
}

fn latch_fs_events(sensor: Res<FsSensor>, mut changed: ResMut<FsChanged>) {
    if let Ok(rx) = sensor.receiver.lock() {
        while rx.try_recv().is_ok() {
            changed.0 = true;
        }
    }
}
