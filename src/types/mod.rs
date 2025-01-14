pub mod body;
pub mod callbacks;
pub mod ccd;
pub mod collisions;
pub mod constraints;
pub mod handles;
pub mod math;
pub mod shapes;
pub mod simulation;
pub mod statics;
pub mod utilities;

#[cfg(target_feature = "avx512f")]
pub const WIDEST_LANE: usize = 16;

#[cfg(all(target_feature = "sse2", not(target_feature = "avx512f")))]
pub const WIDEST_LANE: usize = 8;

#[cfg(not(any(target_feature = "avx512f", target_feature = "sse2")))]
pub const WIDEST_LANE: usize = 4; // Fallback for systems without AVX512F or SSE2
