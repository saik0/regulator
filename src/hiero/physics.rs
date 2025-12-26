//! src/hiero/physics.rs
//!
//! # The Physics Engine Definitions
//!
//! This file defines the **6-Dimensional Coordinate Space** for the Hiero DSL.
//! It serves as the backing type system for the `glyph!` macro.
//!
//! # The "God Word" Optimization
//! The `Glyph` struct is padded to 8 bytes (64-bit). This allows the entire
//! semantic meaning of an entity to be treated as a single `u64` for O(1)
//! equality checks, hashing, and copying.

use bevy_ecs::prelude::Component;
use crate::impl_safe_physics;

/// The core metadata unit within the Regulator system.
///
/// # Layout (64-bit)
/// `[Role, Kind, Force, Temporal, Enforcement, Failure, Ver, Res]`
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(8))]
pub struct Glyph {
    // --- The 6 Semantic Dimensions ---
    pub role: Role,             // Agency (State -> Command)
    pub kind: Kind,             // Complexity (Point -> System)
    pub force: Force,           // Pressure (Law -> Annotation)
    pub temporal: Temporal,     // Entropy (Static -> Chaos)
    pub enforcement: Enforcement, // Automation (Code -> Human)
    pub failure: Failure,       // Severity (Stop -> Discard)

    // --- The Structural Padding ---
    pub version: u8,           // 0x01
    pub reserved: u8,          // 0x00
}

// ─────────────────────────────────────────────────────────────
// Dimension 1: Role (Agency: Mass -> Energy)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    State = 0,    // Inert Mass (Fact)
    Witness = 1,  // Passive Observer (Sensor)
    Lens = 2,     // Passive Filter (View)
    Gate = 3,     // Active Regulator (Limit)
    Catalyst = 4, // Active Trigger (Spark)
    Command = 5,  // Imperative Energy (Verb)
}

// ─────────────────────────────────────────────────────────────
// Dimension 2: Kind (Complexity: Point -> System)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    // Atomic
    Truth = 0,
    Metric = 1,
    Orientation = 2,
    // Structural
    Boundary = 3,
    Relationship = 4,
    Lifecycle = 5,
    // Dynamic
    Transformation = 6,
    Process = 7,
    Control = 8,
}

// ─────────────────────────────────────────────────────────────
// Dimension 3: Force (Pressure: Hard -> Soft)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Force {
    Law = 0,        // Panic
    Constraint = 1, // Error
    Signal = 2,     // Warn
    Annotation = 3, // Info
}

// ─────────────────────────────────────────────────────────────
// Dimension 4: Temporal (Entropy: Stability -> Chaos)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Temporal {
    Static = 0,    // Invariant ($df/dt = 0$)
    Ongoing = 1,   // Continuous ($df/dt = C$)
    Monotonic = 2, // Ordered ($T_{n+1} > T_n$)
    Event = 3,     // Discrete Spike (Impulse)
    Emergent = 4,  // Complex/Unpredictable (Noise)
}

// ─────────────────────────────────────────────────────────────
// Dimension 5: Enforcement (Trust: Machine -> Human)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Enforcement {
    Mechanical = 0, // Trusted (Compiled)
    Hybrid = 1,     // Augmented (Signed)
    Human = 2,      // Manual (Subjective)
    Never = 3,      // Untrusted (Raw)
}

// ─────────────────────────────────────────────────────────────
// Dimension 6: Failure (Severity: Death -> Noise)
// ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Failure {
    Stop = 0,           // System Halt
    Illegal = 1,        // Context Halt
    Invalid = 2,        // Logic Error
    Divergence = 3,     // Consistency Error
    Nondeterminism = 4, // Quality Warning
    Drift = 5,          // Accuracy Warning
    Confusion = 6,      // Timeout
    LoudFail = 7,       // Alert
    Discard = 8,        // Ignore
}

impl_safe_physics!(Role, Role::Command);
impl_safe_physics!(Kind, Kind::Control);
impl_safe_physics!(Force, Force::Annotation);
impl_safe_physics!(Temporal, Temporal::Emergent);
impl_safe_physics!(Enforcement, Enforcement::Never);
impl_safe_physics!(Failure, Failure::Discard);

// ─────────────────────────────────────────────────────────────
// The Kinematics (Manhattan Logic)
// ─────────────────────────────────────────────────────────────

impl Glyph {
    /// **Manhattan Distance (Taxicab Metric).**
    ///
    /// Calculates the "Edit Cost" to transform concept A into B.
    /// * Cost = Sum of steps along each axis.
    /// * Integer only. No Sqrt. Extremely fast.
    #[inline]
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> u32 {
        let mut cost = 0u32;

        cost += (self.role as i32 - other.role as i32).unsigned_abs();
        cost += (self.kind as i32 - other.kind as i32).unsigned_abs();
        cost += (self.force as i32 - other.force as i32).unsigned_abs();
        cost += (self.temporal as i32 - other.temporal as i32).unsigned_abs();
        cost += (self.enforcement as i32 - other.enforcement as i32).unsigned_abs();
        cost += (self.failure as i32 - other.failure as i32).unsigned_abs();

        cost
    }

    /// **Discrete Lerp (Rounding).**
    ///
    /// Interpolates between states using standard rounding (add divisor/2).
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)] // Verified: Output range is 0-255
    pub fn lerp(&self, target: Self, t_u8: u8) -> Self {
        let t = u32::from(t_u8);

        // Helper to lerp a single byte with rounding
        let lerp_byte = |a: u8, b: u8| -> u8 {
            let a = u32::from(a);
            let b = u32::from(b);
            if b >= a {
                (a + ((b - a) * t + 127) / 255) as u8
            } else {
                (a - ((a - b) * t + 127) / 255) as u8
            }
        };

        Self {
            role: Role::from_u8(lerp_byte(self.role as u8, target.role as u8)),
            kind: Kind::from_u8(lerp_byte(self.kind as u8, target.kind as u8)),
            force: Force::from_u8(lerp_byte(self.force as u8, target.force as u8)),
            temporal: Temporal::from_u8(lerp_byte(self.temporal as u8, target.temporal as u8)),
            enforcement: Enforcement::from_u8(lerp_byte(self.enforcement as u8, target.enforcement as u8)),
            failure: Failure::from_u8(lerp_byte(self.failure as u8, target.failure as u8)),

            version: self.version,
            reserved: 0,
        }
    }
}