// Copyright 2017-2019 Emma Welker (nuew)
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

const BIN: &[&[u8]] = &[
    include_bytes!("common/data/ignoreGarbage/abc.bin"),
    include_bytes!("common/data/ignoreGarbage/continuationAtEnd.bin"),
    include_bytes!("common/data/ignoreGarbage/lineBreak.bin"),
    include_bytes!("common/data/ignoreGarbage/lineBreaks.bin"),
    include_bytes!("common/data/ignoreGarbage/quoted.bin"),
    include_bytes!("common/data/ignoreGarbage/randomAlphanumericInterference.bin"),
    include_bytes!("common/data/ignoreGarbage/spaceAfter.bin"),
    include_bytes!("common/data/ignoreGarbage/spaceBefore.bin"),
    include_bytes!("common/data/ignoreGarbage/spacesEverywhere.bin"),
];

const TXT: &[&str] = &[
    include_str!("common/data/ignoreGarbage/abc.txt"),
    include_str!("common/data/ignoreGarbage/continuationAtEnd.txt"),
    include_str!("common/data/ignoreGarbage/lineBreak.txt"),
    include_str!("common/data/ignoreGarbage/lineBreaks.txt"),
    include_str!("common/data/ignoreGarbage/quoted.txt"),
    include_str!("common/data/ignoreGarbage/randomAlphanumericInterference.txt"),
    include_str!("common/data/ignoreGarbage/spaceAfter.txt"),
    include_str!("common/data/ignoreGarbage/spaceBefore.txt"),
    include_str!("common/data/ignoreGarbage/spacesEverywhere.txt"),
];

#[test]
fn sanity() {
    assert_eq!(BIN.len(), TXT.len());
}

#[test]
fn decode() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        assert_eq!(
            super::decode(input, true).unwrap(),
            expected,
            "Failed at i = {}",
            i
        );
    }
}

#[test]
fn decode_buf() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        let mut buf = Vec::new();
        super::decode_buf(input, &mut buf, true).unwrap();
        assert_eq!(buf, expected, "Failed at i = {}", i);
    }
}

#[test]
fn decode_slice() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        let mut buf = vec![0; expected.len()].into_boxed_slice();
        super::decode_slice(input, &mut buf, true).unwrap();
        assert_eq!(&*buf, expected, "Failed at i = {}", i);
    }
}
