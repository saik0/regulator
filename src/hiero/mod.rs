// src/hiero/mod.rs

pub mod canon;
pub mod macros;
pub mod physics;

pub use canon::*;
pub use physics::Glyph;

use std::fmt;

#[allow(unreachable_patterns, unused, non_snake_case)] // Work in Progress!
impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Reverse Lookup: Match the 6-byte data against the Canon constants.
        // This is where the data regains its "Identity."
        let (sigil, label) = match *self {
            FACT => ("🪨", "FACT"),
            WITNESS_TOKEN => ("🪙", "WITNESS_TOKEN"),
            SINGULARITY => ("🧿", "SINGULARITY"),
            ALGEBRA => ("🧮", "ALGEBRA"),
            INVARIANT => ("🧷", "INVARIANT"),
            MEMBRANE => ("⛩️", "MEMBRANE"),
            PROHIBIT => ("🚫", "PROHIBIT"),
            DISCARD => ("🗑️", "DISCARD"),
            ENTROPY => ("🕸️", "ENTROPY"),
            NORMALIZE => ("🧘", "NORMALIZE"),
            EXECUTE => ("▶️", "EXECUTE"),
            REPLAY => ("🔁", "REPLAY"),
            PRESSURE => ("🌡️", "PRESSURE"),
            FRAMED => ("🖼️", "FRAMED"),
            MANIFOLD => ("🌐", "MANIFOLD"),
            _ => ("❓", "UNKNOWN_COORDINATE"),
        };

        if f.alternate() {
            // {:#} format: Only the Sigil (e.g., "🪨")
            write!(f, "{sigil}")
        } else {
            // Standard format: Sigil + Label (e.g., "🪨 FACT")
            write!(f, "{sigil} {label}")
        }
    }
}

impl fmt::Debug for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // High-bandwidth debug view: "🪨 FACT [R:0, K:0, F:0, T:0, E:0, F:0]"
        write!(
            f,
            "{} [R:{:?}, K:{:?}, F:{:?}, T:{:?}, E:{:?}, F:{:?}]",
            self, // Calls Display
            self.role as u8,
            self.kind as u8,
            self.force as u8,
            self.temporal as u8,
            self.enforcement as u8,
            self.failure as u8
        )
    }
}
