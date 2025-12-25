//! The Physics Engine definitions.
//! This file defines the 6-dimensional coordinate space for the Hiero DSL.
//! It is the backing type system for the glyph! macro.

use crate::impl_safe_physics;
use bevy_ecs::prelude::Component;

/// The core metadata unit within the Regulator system.
/// Each coordinate represents a fixed point in a 6-dimensional semantic space.
/// Packed into 6 bytes for maximum cache efficiency in ECS queries.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Glyph {
    /// 1. Role: The functional geometry (What is its intent?)
    pub role: Role,
    /// 2. Kind: The ontological category (What is this?)
    pub kind: Kind,
    /// 3. Force: The binding pressure (How strict is this?)
    pub force: Force,
    /// 4. Temporal: The relationship to time (When does this apply?)
    pub temporal: Temporal,
    /// 5. Enforcement: The authority of the watcher (Who checks this?)
    pub enforcement: Enforcement,
    /// 6. Failure: The mode of systemic collapse (How does it break?)
    pub failure: Failure,
}

// ─────────────────────────────────────────────────────────────
// Dimension 1: Role (Syntactic Geometry)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Noun,     // A discrete object or state-holder (Fact, Token).
    Verb,     // An action or state-mutator (Execute, Normalize).
    Gate,     // A conditional barrier that regulates flow (Membrane).
    Lens,     // A perspective that filters or transforms (Framed, Manifold).
    Witness,  // An observer that validates or logs state.
    Catalyst, // A trigger that accelerates or initiates change.
}

// ─────────────────────────────────────────────────────────────
// Dimension 2: Kind (Ontology)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Truth,          // Absolute, immutable facts.
    Transformation, // Active state transitions.
    Process,        // A sequence of orchestrated steps.
    Control,        // Regulatory mechanisms and steering logic.
    Boundary,       // Edges between safe and unsafe contexts.
    Metric,         // Measurements of system health.
    Orientation,    // Positional or directional metadata.
    Lifecycle,      // Birth, growth, and terminal states.
    Relationship,   // Connections between entities.
}

// ─────────────────────────────────────────────────────────────
// Dimension 3: Force (Binding Pressure)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Force {
    Law,        // Inviolable system requirements (Panic/Stop).
    Constraint, // Boundary conditions that limit the solution space.
    Signal,     // Advisory indications or soft triggers.
    Annotation, // Pure metadata with no operational impact.
}

// ─────────────────────────────────────────────────────────────
// Dimension 4: Temporal (Time Relationship)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Temporal {
    Static,    // Constant across all time-steps.
    Monotonic, // Always increasing or moving forward.
    Event,     // Occurs at a discrete point in time.
    Ongoing,   // Continuous or persistent over an interval.
    Emergent,  // Spontaneously arising from system complexity.
}

// ─────────────────────────────────────────────────────────────
// Dimension 5: Enforcement (Authority)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enforcement {
    Mechanical, // Validated by hardcoded logic or compiler checks.
    Human,      // Requires manual intervention or subjective judgment.
    Hybrid,     // A combination of automated triggers and human sign-off.
    Never,      // Explicitly unverified and untrusted.
}

// ─────────────────────────────────────────────────────────────
// Dimension 6: Failure (Collapse Mode)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure {
    Stop,           // Immediate halt of all system processes.
    Invalid,        // The state is logically impossible or corrupted.
    LoudFail,       // Failure is broadcasted loudly to all observers.
    Nondeterminism, // Results vary despite identical inputs.
    Confusion,      // Ambiguity in intent leads to system paralysis.
    Divergence,     // System branches into incompatible states.
    Drift,          // Subtle movement away from the intended target.
    Illegal,        // Violation of core safety or legal protocols.
}

// ─────────────────────────────────────────────────────────────
// The Physical Boundaries (Safe Snapping)
// ─────────────────────────────────────────────────────────────

impl_safe_physics!(Role, Role::Catalyst);
impl_safe_physics!(Kind, Kind::Relationship);
impl_safe_physics!(Force, Force::Annotation);
impl_safe_physics!(Temporal, Temporal::Emergent);
impl_safe_physics!(Enforcement, Enforcement::Never);
impl_safe_physics!(Failure, Failure::Illegal);

// ─────────────────────────────────────────────────────────────
// The Kinematics (Metabolic Math)
// ─────────────────────────────────────────────────────────────

impl Glyph {
    /// Zero-cost identity distance using integer math.
    /// Returns f32 for compatibility with Bevy transform scales/weights.
    #[inline]
    #[must_use] 
    pub fn distance_to(&self, other: &Self) -> f32 {
        let a = [
            self.role as i32,
            self.kind as i32,
            self.force as i32,
            self.temporal as i32,
            self.enforcement as i32,
            self.failure as i32,
        ];
        let b = [
            other.role as i32,
            other.kind as i32,
            other.force as i32,
            other.temporal as i32,
            other.enforcement as i32,
            other.failure as i32,
        ];

        let mut sum = 0;
        for i in 0..6 {
            let diff = a[i] - b[i];
            sum += diff * diff;
        }
        (sum as f32).sqrt()
    }

    /// Fixed-Point Lerp: Discrete transition across the manifold.
    /// `t_u8`: 0 (0%) to 255 (100%).
    #[inline]
    #[must_use] 
    pub fn lerp(&self, target: Self, t_u8: u8) -> Self {
        let t = u32::from(t_u8);
        let a = [
            self.role as u32,
            self.kind as u32,
            self.force as u32,
            self.temporal as u32,
            self.enforcement as u32,
            self.failure as u32,
        ];
        let b = [
            target.role as u32,
            target.kind as u32,
            target.force as u32,
            target.temporal as u32,
            target.enforcement as u32,
            target.failure as u32,
        ];

        let mut res = [0u8; 6];
        for i in 0..6 {
            res[i] = if b[i] >= a[i] {
                (a[i] + ((b[i] - a[i]) * t) / 255) as u8
            } else {
                (a[i] - ((a[i] - b[i]) * t) / 255) as u8
            };
        }

        Self {
            role: Role::from_u8(res[0]),
            kind: Kind::from_u8(res[1]),
            force: Force::from_u8(res[2]),
            temporal: Temporal::from_u8(res[3]),
            enforcement: Enforcement::from_u8(res[4]),
            failure: Failure::from_u8(res[5]),
        }
    }
}
