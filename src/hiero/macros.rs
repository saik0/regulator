// src/hiero/macros.rs

#[macro_export]
macro_rules! glyph {
    ($name:ident, $sigil:expr, $phonetic:expr, $doc:expr, $data:expr) => {
        #[doc = $doc]
        #[doc = ""]
        #[doc = "### Phonetic"]
        #[doc = $phonetic]
        #[doc = ""]
        #[doc = "### Physics"]
        #[doc = "6-dimensional discrete manifold coordinate."]
        #[allow(non_camel_case_types)] // Work in Progress!
        pub trait $name {
            const SIGIL: &'static str = $sigil;
            const LABEL: &'static str = stringify!($name);
            const PHONETIC: &'static str = $phonetic;
        }

        impl $name for $crate::hiero::physics::Glyph {}

        #[doc = $doc]
        #[allow(non_camel_case_types)] // Work in Progress!
        pub const $name: $crate::hiero::physics::Glyph = $crate::hiero::physics::Glyph {
            role: $data.role,
            kind: $data.kind,
            force: $data.force,
            temporal: $data.temporal,
            enforcement: $data.enforcement,
            failure: $data.failure,
        };
    };
}

/// Enforces the "Physical Boundary" of a dimension.
///
/// This macro generates a `from_u8` constructor for `repr(u8)` enums.
/// It provides a zero-cost transition from raw metabolic math back into
/// the safe, named types of the Hiero system.
///
/// # Logic
/// 1. If the input `v` is within the valid variant range, it uses a
///    high-speed bit-copy (transmute).
/// 2. If `v` exceeds the range (e.g., during an aggressive lerp), it
///    snaps the value to the `$max` variant (the "Safety Rail").
///
/// # Safety
/// This is safe because it explicitly checks bounds before performing
/// the transmute. It prevents Undefined Behavior (UB) caused by
/// "Invalid Discriminants."
#[macro_export]
macro_rules! impl_safe_physics {
    ($t:ty, $max:expr) => {
        impl $t {
            /// Safely snaps a raw byte to the nearest valid enum variant.
            #[inline]
            pub fn from_u8(v: u8) -> Self {
                // The compiler optimizes this into a branchless CMOV instruction.
                if v <= ($max as u8) {
                    // SAFETY: Bound is checked above; value is guaranteed
                    // to be a valid bit-pattern for this repr(u8) enum.
                    unsafe { std::mem::transmute::<u8, $t>(v) }
                } else {
                    $max
                }
            }
        }
    };
}
