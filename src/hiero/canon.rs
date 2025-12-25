#![allow(clippy::doc_markdown)] // Work in Progress!
//! src/hiero/canon.rs
//! The authoritative collection of Archetypal Glyphs.
//! These act as the fixed "Stars" in the 6D discrete coordinate system.

use crate::glyph;
use crate::hiero::physics::{Enforcement, Failure, Force, Glyph, Kind, Role, Temporal};

// ─────────────────────────────────────────────────────────────
// 1. THE SUBSTRATE (Hard State)
// ─────────────────────────────────────────────────────────────

glyph!(
    FACT,
    "🪨",
    "fakt",
    "Immutable truth read from the journal. If this is wrong, the Regulator must stop.",
    Glyph {
        role: Role::Noun,
        kind: Kind::Truth,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
    }
);

glyph!(
    TOKEN,
    "🪙",
    "ˈtōkən",
    "A minted record of a validated state (Proof of Reason). Used to anchor historical truth.",
    Glyph {
        role: Role::Witness,
        kind: Kind::Truth,
        force: Force::Signal,
        temporal: Temporal::Event,
        enforcement: Enforcement::Hybrid,
        failure: Failure::LoudFail,
    }
);

glyph!(
    SINGULARITY,
    "🧿",
    "ˌsiNGɡyəˈlerədē",
    "The point of maximum metabolic pressure where intent, logic, and hardware align.",
    Glyph {
        role: Role::Catalyst,
        kind: Kind::Transformation,
        force: Force::Law,
        temporal: Temporal::Emergent,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Illegal,
    }
);

// ─────────────────────────────────────────────────────────────
// 2. THE LAWS (Logic & Math)
// ─────────────────────────────────────────────────────────────

glyph!(
    ALGEBRA,
    "🧮",
    "ˈaljəbrə",
    "Canonical transformations and composition rules for validating logical equivalence.",
    Glyph {
        role: Role::Noun,
        kind: Kind::Transformation,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Invalid,
    }
);

glyph!(
    INVARIANT,
    "🧷",
    "inˈverēənt",
    "A constraint that must never be violated. Must be verified before minting tokens.",
    Glyph {
        role: Role::Noun,
        kind: Kind::Truth,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Hybrid,
        failure: Failure::Stop,
    }
);

glyph!(
    VOID,
    "🚪",
    "void",
    "The boundary between the Unsafe World and the System. Data here MUST be normalized.",
    Glyph {
        role: Role::Gate,
        kind: Kind::Boundary,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Invalid,
    }
);

// ─────────────────────────────────────────────────────────────
// 3. THE DECISION (Control Flow)
// ─────────────────────────────────────────────────────────────

glyph!(
    PROHIBIT,
    "🚫",
    "prōˈhibət",
    "Explicit 'do not do this'. Triggers a panic if the system attempts to cross.",
    Glyph {
        role: Role::Verb,
        kind: Kind::Control,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::LoudFail,
    }
);

glyph!(
    DISCARD,
    "🗑️",
    "diˈskärd",
    "Valid but uninteresting data. Consciously ignored to preserve metabolic bandwidth.",
    Glyph {
        role: Role::Verb,
        kind: Kind::Control,
        force: Force::Constraint,
        temporal: Temporal::Event,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Invalid,
    }
);

glyph!(
    ENTROPY,
    "🕸️",
    "ˈentrəpē",
    "Unstructured noise or chaos used to test the resilience of Invariants.",
    Glyph {
        role: Role::Noun,
        kind: Kind::Metric,
        force: Force::Signal,
        temporal: Temporal::Emergent,
        enforcement: Enforcement::Never,
        failure: Failure::Confusion,
    }
);

// ─────────────────────────────────────────────────────────────
// 4. THE PROCESS (Runtime Motion)
// ─────────────────────────────────────────────────────────────

glyph!(
    NORMALIZE,
    "🧘",
    "ˈnôrməˌlīz",
    "Reduce a problem to a centered, stable form to prevent operational thrash.",
    Glyph {
        role: Role::Verb,
        kind: Kind::Transformation,
        force: Force::Constraint,
        temporal: Temporal::Event,
        enforcement: Enforcement::Human,
        failure: Failure::Confusion,
    }
);

glyph!(
    EXECUTE,
    "▶️",
    "ˈeksəˌkyo͞ot",
    "Apply facts to produce state. If results diverge from replay, the token is void.",
    Glyph {
        role: Role::Verb,
        kind: Kind::Process,
        force: Force::Constraint,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Divergence,
    }
);

glyph!(
    REPLAY,
    "🔁",
    "rēˈplā",
    "Reconstruct behavior from historical facts. Verification, not storytelling.",
    Glyph {
        role: Role::Verb,
        kind: Kind::Process,
        force: Force::Constraint,
        temporal: Temporal::Event,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Nondeterminism,
    }
);

// ─────────────────────────────────────────────────────────────
// 5. THE PHYSICS (Environment & Health)
// ─────────────────────────────────────────────────────────────

glyph!(
    PRESSURE,
    "🌡️",
    "ˈpreSHər",
    "The operational cost (RAM/CPU). High pressure forces load-shedding.",
    Glyph {
        role: Role::Noun,
        kind: Kind::Metric,
        force: Force::Signal,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Hybrid,
        failure: Failure::LoudFail,
    }
);

glyph!(
    FRAME,
    "🖼️",
    "frāmd",
    "A witness that a specific scope was chosen for the current context.",
    Glyph {
        role: Role::Lens,
        kind: Kind::Orientation,
        force: Force::Annotation,
        temporal: Temporal::Event,
        enforcement: Enforcement::Never,
        failure: Failure::Confusion,
    }
);

glyph!(
    ANOMALY,
    "⚠️",
    "əˈnäməlē",
    "A state of high divergence where logic no longer maps to historical fact.",
    Glyph {
        role: Role::Lens,
        kind: Kind::Orientation,
        force: Force::Annotation,
        temporal: Temporal::Emergent,
        enforcement: Enforcement::Never,
        failure: Failure::Divergence,
    }
);

// ─────────────────────────────────────────────────────────────
// 6. THE MANIFOLD (The Space)
// ─────────────────────────────────────────────────────────────

glyph!(
    MANIFOLD,
    "🌐",
    "ˈmanəˌfōld",
    "The full coordinate space representing the cybernetic system's health.",
    Glyph {
        role: Role::Lens,
        kind: Kind::Control,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
    }
);

glyph!(
    FLOW,
    "🌊",
    "flō",
    "Represents an ongoing lifecycle process with high temporal fluidity.",
    Glyph {
        role: Role::Verb,
        kind: Kind::Lifecycle,
        force: Force::Constraint,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Human,
        failure: Failure::Drift,
    }
);
