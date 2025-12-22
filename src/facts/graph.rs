use bevy_ecs::prelude::*;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{BTreeMap, HashSet};

use crate::facts::ids::{DagId, TaskId};
use crate::facts::task::Status;

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

/// The pure algebra of task readiness.
/// Decoupled from ECS to allow for property-based testing and Indexer reuse.
#[must_use]
pub fn calculate_ready_tasks(tasks: &[(TaskId, Status)], graph: &TaskGraph) -> Vec<TaskId> {
    let finished: HashSet<TaskId> = tasks
        .iter()
        .filter(|(_, s)| matches!(s, Status::Complete | Status::Cancelled))
        .map(|(id, _)| *id)
        .collect();

    tasks
        .iter()
        .filter(|(id, s)| {
            matches!(s, Status::Pending)
                && graph.dependencies(*id).iter().all(|d| finished.contains(d))
        })
        .map(|(id, _)| *id)
        .collect()
}

/// The Bevy system wrapper that uses the pure logic above.
#[must_use]
pub fn ready_tasks_logic(query: Query<(&TaskId, &Status)>, graph: &TaskGraph) -> Vec<TaskId> {
    let data: Vec<(TaskId, Status)> = query.iter().map(|(id, s)| (*id, s.clone())).collect();

    // DETERMINISM: Sort the output so the metabolism always processes
    // ready tasks in the same order regardless of ECS iteration order.
    let mut ready_tasks = calculate_ready_tasks(&data, graph);
    ready_tasks.sort();
    ready_tasks
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashMap;

    proptest! {
        #[test]
        fn test_task_graph_is_always_dag(
            // Generate a list of (dependent_index, dependency_index)
            edges in prop::collection::vec((0..10usize, 0..10usize), 0..20)
        ) {
            let mut graph = TaskGraph::new(DagId::new("test"));
            let ids: Vec<TaskId> = (0..10).map(|i| TaskId::new(&i.to_string())).collect();

            for id in &ids {
                graph.add_task(*id);
            }

            for (a_idx, b_idx) in edges {
                let res = graph.add_dependency(ids[a_idx], ids[b_idx]);

                // If the addition succeeded, the graph MUST still be acyclic
                if res.is_ok() {
                    prop_assert!(!petgraph::algo::is_cyclic_directed(&graph.graph));
                }
            }
        }

        #[test]
        fn test_ready_tasks_only_have_completed_dependencies(
            // Generate a graph of 10 tasks and random statuses for them
            statuses in prop::collection::vec(
                prop_oneof![
                    Just(Status::Pending),
                    Just(Status::Complete),
                    Just(Status::Cancelled)
                ], 10),
            // Random edges
            edges in prop::collection::vec((0..10usize, 0..10usize), 0..20)
        ) {
            let mut graph = TaskGraph::new(DagId::new("test"));
            let ids: Vec<TaskId> = (0..10).map(|i| TaskId::new(&i.to_string())).collect();
            let mut status_map = HashMap::new();

            for (i, id) in ids.iter().enumerate() {
                graph.add_task(*id);
                status_map.insert(*id, statuses[i].clone());
            }

            for (a, b) in edges {
                let _ = graph.add_dependency(ids[a], ids[b]);
            }

            // We mock the Bevy Query behavior using a simple vector for the logic
            let query_data: Vec<(&TaskId, &Status)> = status_map.iter().collect();

            // Re-implementing the logic body for the test context if needed,
            // or calling a decoupled version of ready_tasks_logic.
            let ready = ready_tasks_logic_standalone(&query_data, &graph);

            for ready_id in ready {
                // Invariant 1: Must be Pending
                prop_assert_eq!(status_map.get(&ready_id).unwrap(), &Status::Pending);

                // Invariant 2: All dependencies must be Complete or Cancelled
                for dep in graph.dependencies(ready_id) {
                    let s = status_map.get(&dep).unwrap();
                    prop_assert!(matches!(s, Status::Complete | Status::Cancelled));
                }
            }
        }

        #[test]
        fn test_readiness_algebra_invariants(
            // Generate random statuses for up to 20 tasks
            statuses in prop::collection::vec(
                prop_oneof![
                    Just(Status::Pending),
                    Just(Status::Active),
                    Just(Status::Complete),
                    Just(Status::Cancelled)
                ], 1..20),
            // Generate random dependency edges
            edges in prop::collection::vec((0..20usize, 0..20usize), 0..30)
        ) {
            let mut graph = TaskGraph::new(DagId::new("test"));
            let ids: Vec<TaskId> = (0..statuses.len())
                .map(|i| TaskId::new(&i.to_string()))
                .collect();

            let mut task_data = Vec::new();
            for (i, id) in ids.iter().enumerate() {
                graph.add_task(*id);
                task_data.push((*id, statuses[i].clone()));
            }

            // Attempt to build a random graph
            for (a, b) in edges {
                if a < ids.len() && b < ids.len() && a != b {
                    let _ = graph.add_dependency(ids[a], ids[b]);
                }
            }

            let ready = calculate_ready_tasks(&task_data, &graph);

            for ready_id in ready {
                let status = task_data.iter().find(|(id, _)| *id == ready_id).unwrap().1.clone();

                // LAW 1: Only Pending tasks can be promoted to Ready
                prop_assert_eq!(status, Status::Pending);

                // LAW 2: All upstream dependencies MUST be terminal (Complete/Cancelled)
                for dep_id in graph.dependencies(ready_id) {
                    let dep_status = task_data.iter().find(|(id, _)| *id == dep_id).unwrap().1.clone();
                    prop_assert!(
                        matches!(dep_status, Status::Complete | Status::Cancelled),
                        "Ready task {:?} has non-terminal dependency {:?} ({:?})",
                        ready_id, dep_id, dep_status
                    );
                }
            }
        }
    }

    // A decoupled version of the logic to make it testable outside the ECS world
    fn ready_tasks_logic_standalone(data: &[(&TaskId, &Status)], graph: &TaskGraph) -> Vec<TaskId> {
        let finished: std::collections::HashSet<TaskId> = data
            .iter()
            .filter(|(_, s)| matches!(s, Status::Complete | Status::Cancelled))
            .map(|(id, _)| **id)
            .collect();

        data.iter()
            .filter(|(id, s)| {
                matches!(s, Status::Pending)
                    && graph
                        .dependencies(**id)
                        .iter()
                        .all(|d| finished.contains(d))
            })
            .map(|(id, _)| **id)
            .collect()
    }
}
