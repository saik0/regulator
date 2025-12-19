use bevy_ecs::prelude::*;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{BTreeMap, HashSet};

use crate::facts::ids::{DagId, Oid, TaskId};
use crate::facts::task::Status;

/// Metadata for the current execution session.
#[derive(Resource)]
pub struct MallSession {
    pub base_oid: Oid,
    pub base_branch: String,
    pub started_at: std::time::SystemTime,
}

/// A Directed Acyclic Graph (DAG) representing task dependencies.

#[derive(Resource)]
pub struct TaskGraph {
    pub dag_id: DagId,
    pub(crate) graph: DiGraph<TaskId, ()>,
    pub(crate) index: BTreeMap<TaskId, NodeIndex>,
}

impl TaskGraph {
    /// Creates a new, empty `TaskGraph`.
    #[must_use]
    pub fn new(dag_id: DagId) -> Self {
        Self {
            dag_id,
            graph: DiGraph::new(),
            index: BTreeMap::new(),
        }
    }

    /// Registers a task as a node in the graph.
    pub fn add_task(&mut self, task_id: TaskId) {
        let node = self.graph.add_node(task_id);
        self.index.insert(task_id, node);
    }

    /// Links two tasks, establishing that `dependent` requires `dependency`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - Either the `dependent` or `dependency` task has not been added to the graph.
    /// - Adding the edge would introduce a cycle (violating DAG constraints).
    pub fn add_dependency(&mut self, dependent: TaskId, dependency: TaskId) -> Result<(), String> {
        let dep_node = self.index.get(&dependent).ok_or("Dependent not found")?;
        let src_node = self.index.get(&dependency).ok_or("Dependency not found")?;

        let edge = self.graph.add_edge(*src_node, *dep_node, ());

        if petgraph::algo::is_cyclic_directed(&self.graph) {
            self.graph.remove_edge(edge);
            return Err("Cycle detected: dependency would create a loop".into());
        }
        Ok(())
    }

    /// Returns a list of all immediate dependencies for a given task.
    #[must_use]
    pub fn dependencies(&self, id: TaskId) -> Vec<TaskId> {
        self.index
            .get(&id)
            .map(|idx| {
                self.graph
                    .neighbors_directed(*idx, petgraph::Direction::Incoming)
                    .filter_map(|n| self.graph.node_weight(n))
                    .copied()
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Logic to identify tasks that are `Pending` and have all dependencies `Complete`.
#[must_use]
pub fn ready_tasks_logic(query: Query<(&TaskId, &Status)>, graph: &TaskGraph) -> Vec<TaskId> {
    let finished: HashSet<TaskId> = query
        .iter()
        .filter(|(_, s)| matches!(s, Status::Complete | Status::Cancelled))
        .map(|(id, _)| *id)
        .collect();

    query
        .iter()
        .filter(|(id, s)| {
            matches!(s, Status::Pending)
                && graph
                    .dependencies(**id)
                    .iter()
                    .all(|d| finished.contains(d))
        })
        .map(|(id, _)| *id)
        .collect()
}
