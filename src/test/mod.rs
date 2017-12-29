// Copyright 2017 Ethan Welker (nuew)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
fn correct_b2s_types() {
    #[cfg(feature = "fnv")]
    let _: &::fnv::FnvBuildHasher = BLOCK_START_TO_INDEX.hasher();
    #[cfg(not(feature = "fnv"))]
    let _: &::std::collections::hash_map::RandomState = BLOCK_START_TO_INDEX.hasher();
}

#[test]
fn error_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Error>();
}

#[test]
fn error_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Error>();
}

#[test]
fn wrapoptions_send() {
    fn assert_send<T: Send>() {}
    assert_send::<WrapOptions>();
}

#[test]
fn wrapoptions_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<WrapOptions>();
}
