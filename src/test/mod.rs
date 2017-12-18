use super::*;

mod bad;
mod doubled_bytes;
mod ignore_garbage;
mod pairs;
mod single_bytes;
mod wrap;

#[cfg(feature = "nightly")]
mod bench;

#[test]
// this doesn't really fit anywhere else
fn correct_b2s_types() {
    #[cfg(feature = "fnv")]
    let _: &::fnv::FnvBuildHasher = BLOCK_START_TO_INDEX.hasher();
    #[cfg(not(feature = "fnv"))]
    let _: &::std::collections::hash_map::RandomState = BLOCK_START_TO_INDEX.hasher();
}
