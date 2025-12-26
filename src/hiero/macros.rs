#[macro_export]
macro_rules! glyph {
    ($name:ident, $sigil:expr, $phonetic:expr, $doc:expr, $data:expr) => {
        #[doc = $doc]
        #[doc = ""]
        #[doc = "---"]
        #[doc = "### 🗣️ Phonetic"]
        #[doc = concat!("`/", $phonetic, "/`")]
        #[doc = ""]
        #[doc = "### 📐 Physics"]
        #[doc = concat!("**Sigil:** ", $sigil)]
        #[doc = ""]
        #[doc = "**Manifold Coordinate (64-bit Aligned):**"]
        #[doc = "This constant is a fixed point in the discrete semantic space."]
        #[allow(non_camel_case_types)]
        pub trait $name {
            const SIGIL: &'static str = $sigil;
            const LABEL: &'static str = stringify!($name);
            const PHONETIC: &'static str = $phonetic;
        }

        // Allow the Glyph struct to act as a witness for this Archetype
        impl $name for $crate::hiero::physics::Glyph {}

        #[doc = concat!("The canonical **", stringify!($name), "** Archetype.")]
        #[doc = ""]
        #[doc = "Contains the specific 6D coordinate + padding bytes."]
        #[allow(non_camel_case_types)]
        pub const $name: $crate::hiero::physics::Glyph = $data;
    };
}


/// Generates `from_u8` that snaps unsafe bytes to the nearest valid physical constant.
/// This prevents undefined behavior when lerping produces "out of bounds" bytes.
#[macro_export]
macro_rules! impl_safe_physics {
    ($type:ty, $max_variant:expr) => {
        impl $type {
            #[inline(always)]
            pub fn from_u8(val: u8) -> Self {
                // SAFETY: We camp the input to the max variant discriminator.
                // Since our enums are repr(u8) and continuous from 0..=MAX,
                // this transmute is strictly safe.
                let clamped = val.min($max_variant as u8);
                unsafe { std::mem::transmute(clamped) }
            }
        }
    };
}