#![allow(clippy::doc_markdown)]
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
        role: Role::State,
        kind: Kind::Truth,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    SPARK,
    "⚡",
    "spärk",
    "A transient proposal or impulse (Step<Proposal>). Validated but not yet hardened into history.",
    Glyph {
        role: Role::Catalyst,
        kind: Kind::Process,
        force: Force::Signal,
        temporal: Temporal::Event,
        enforcement: Enforcement::Hybrid,
        failure: Failure::Discard,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    CELL,
    "🦠",
    "sel",
    "The atomic unit of metabolic potential. A Fact viewed as a living input to the organism.",
    Glyph {
        role: Role::State,
        kind: Kind::Lifecycle,
        force: Force::Constraint,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Never,
        failure: Failure::Confusion,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    ENVELOPE,
    "✉️",
    "ˈenvəˌlōp",
    "The 7-dimensional tuple container (UUID, Type, Value, Tx, Vs, Ve, Op).",
    Glyph {
        role: Role::State,
        kind: Kind::Boundary,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Invalid,
        version: 1,
        reserved: 0,
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
        version: 1,
        reserved: 0,
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
        version: 1,
        reserved: 0,
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
        role: Role::State,
        kind: Kind::Transformation,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Invalid,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    INVARIANT,
    "🧷",
    "inˈverēənt",
    "A constraint that must never be violated. Must be verified before minting tokens.",
    Glyph {
        role: Role::Gate,
        kind: Kind::Truth,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Hybrid,
        failure: Failure::Stop,
        version: 1,
        reserved: 0,
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
        version: 1,
        reserved: 0,
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
        role: Role::Command,
        kind: Kind::Control,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::LoudFail,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    DISCARD,
    "🗑️",
    "diˈskärd",
    "Valid but uninteresting data. Consciously ignored to preserve metabolic bandwidth.",
    Glyph {
        role: Role::Command,
        kind: Kind::Control,
        force: Force::Constraint,
        temporal: Temporal::Event,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Invalid,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    ENTROPY,
    "🕸️",
    "ˈentrəpē",
    "Unstructured noise or chaos used to test the resilience of Invariants.",
    Glyph {
        role: Role::State,
        kind: Kind::Metric,
        force: Force::Signal,
        temporal: Temporal::Emergent,
        enforcement: Enforcement::Never,
        failure: Failure::Confusion,
        version: 1,
        reserved: 0,
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
        role: Role::Command,
        kind: Kind::Transformation,
        force: Force::Constraint,
        temporal: Temporal::Event,
        enforcement: Enforcement::Human,
        failure: Failure::Confusion,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    EXECUTE,
    "▶️",
    "ˈeksəˌkyo͞ot",
    "Apply facts to produce state. If results diverge from replay, the token is void.",
    Glyph {
        role: Role::Command,
        kind: Kind::Process,
        force: Force::Constraint,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Divergence,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    REPLAY,
    "🔁",
    "rēˈplā",
    "Reconstruct behavior from historical facts. Verification, not storytelling.",
    Glyph {
        role: Role::Command,
        kind: Kind::Process,
        force: Force::Constraint,
        temporal: Temporal::Event,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Nondeterminism,
        version: 1,
        reserved: 0,
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
        role: Role::Witness,
        kind: Kind::Metric,
        force: Force::Signal,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Hybrid,
        failure: Failure::LoudFail,
        version: 1,
        reserved: 0,
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
        version: 1,
        reserved: 0,
    }
);

glyph!(
    ANOMALY,
    "⚠️",
    "əˈnäməlē",
    "A state of high divergence where logic no longer maps to historical fact.",
    Glyph {
        role: Role::State,
        kind: Kind::Orientation,
        force: Force::Annotation,
        temporal: Temporal::Emergent,
        enforcement: Enforcement::Never,
        failure: Failure::Divergence,
        version: 1,
        reserved: 0,
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
        version: 1,
        reserved: 0,
    }
);

glyph!(
    FLOW,
    "🌊",
    "flō",
    "Represents an ongoing lifecycle process with high temporal fluidity.",
    Glyph {
        role: Role::Command,
        kind: Kind::Lifecycle,
        force: Force::Constraint,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Human,
        failure: Failure::Drift,
        version: 1,
        reserved: 0,
    }
);

// ─────────────────────────────────────────────────────────────
// 7. THE ORGANISM (Cybernetics)
// ─────────────────────────────────────────────────────────────

glyph!(
    MIRROR,
    "🪞",
    "ˈmirər",
    "The Reflexive Lens. The active projection of the system's state used for self-correction.",
    Glyph {
        role: Role::Lens,
        kind: Kind::Orientation,
        force: Force::Law,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Divergence,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    LINEAGE,
    "🧬",
    "ˈlinēij",
    "The Ancestral Chain. An immutable, append-only record of structural evolution and provenance.",
    Glyph {
        role: Role::State,
        kind: Kind::Truth,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    HEART,
    "🫀",
    "härt",
    "The Controller. It pumps the Tn pulse that synchronizes the organism.",
    Glyph {
        role: Role::Catalyst,
        kind: Kind::Process,
        force: Force::Law,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    MEDULLA,
    "⚖️",
    "məˈdələ",
    "Rust-Analyzer (Left Brain). Enforces homeostasis, types, and algebra.",
    Glyph {
        role: Role::Gate,
        kind: Kind::Control,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    HAND,
    "👐",
    "hand",
    "Agentic LLM (Right Brain). The effector that manipulates intent into code.",
    Glyph {
        role: Role::Command,
        kind: Kind::Transformation,
        force: Force::Signal,
        temporal: Temporal::Event,
        enforcement: Enforcement::Hybrid,
        failure: Failure::LoudFail,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    SENSOR,
    "👁️",
    "ˈsensər",
    "Proprioception (LSP). Feels the lexical pressure and semantic tension.",
    Glyph {
        role: Role::Witness,
        kind: Kind::Metric,
        force: Force::Signal,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Confusion,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    LOOP,
    "🔄",
    "lo͞op",
    "Active Inference. The cycle of Sensing, Acting, and Verifying to minimize surprise.",
    Glyph {
        role: Role::Command,
        kind: Kind::Lifecycle,
        force: Force::Constraint,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Drift,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    GOVERNOR,
    "🎡",
    "ˈɡəvərnər",
    "The Kinetic Bound. Mechanically arrests the system to maintain ethical equilibrium.",
    Glyph {
        role: Role::Gate,
        kind: Kind::Control,
        force: Force::Law,
        temporal: Temporal::Static,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Stop,
        version: 1,
        reserved: 0,
    }
);

glyph!(
    REGULATOR,
    "🎛️",
    "ˈreɡyəˌlādər",
    "The Cybernetic Steersman. The self-referential agent that orients the system towards True North.",
    Glyph {
        role: Role::Lens,
        kind: Kind::Orientation,
        force: Force::Law,
        temporal: Temporal::Ongoing,
        enforcement: Enforcement::Mechanical,
        failure: Failure::Drift,
        version: 1,
        reserved: 0,
    }
);