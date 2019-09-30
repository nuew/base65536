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
    include_bytes!("common/data/pairs/demo.bin"),
    include_bytes!("common/data/pairs/empty.bin"),
    include_bytes!("common/data/pairs/firstDefect.bin"),
    include_bytes!("common/data/pairs/hatetris-wrs/hatetris-wr-rle.bin"),
    include_bytes!("common/data/pairs/hatetris-wrs/hatetris-wr-rle2.bin"),
    include_bytes!("common/data/pairs/hatetris-wrs/hatetris-wr.bin"),
    include_bytes!("common/data/pairs/sample-files/everyByte.bin"),
    include_bytes!("common/data/pairs/sample-files/everyPairOfBytes.bin"),
    include_bytes!("common/data/pairs/sample-files/lena_std.tif.bin"),
];

const TXT: &[&str] = &[
    include_str!("common/data/pairs/demo.txt"),
    include_str!("common/data/pairs/empty.txt"),
    include_str!("common/data/pairs/firstDefect.txt"),
    include_str!("common/data/pairs/hatetris-wrs/hatetris-wr-rle.txt"),
    include_str!("common/data/pairs/hatetris-wrs/hatetris-wr-rle2.txt"),
    include_str!("common/data/pairs/hatetris-wrs/hatetris-wr.txt"),
    include_str!("common/data/pairs/sample-files/everyByte.txt"),
    include_str!("common/data/pairs/sample-files/everyPairOfBytes.txt"),
    include_str!("common/data/pairs/sample-files/lena_std.tif.txt"),
];

#[test]
fn sanity() {
    assert_eq!(BIN.len(), TXT.len());
}

#[test]
fn encode() {
    for i in 0..BIN.len() {
        let input = BIN[i];
        let expected = TXT[i];

        assert_eq!(super::encode(input, None), expected, "Failed at i = {}", i);
    }
}

#[test]
fn encode_buf() {
    for i in 0..BIN.len() {
        let input = BIN[i];
        let expected = TXT[i];

        let mut buf = String::new();
        super::encode_buf(input, &mut buf, None);
        assert_eq!(buf, expected, "Failed at i = {}", i);
    }
}

#[test]
fn decode() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        assert_eq!(
            super::decode(input, false).unwrap(),
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
        super::decode_buf(input, &mut buf, false).unwrap();
        assert_eq!(buf, expected, "Failed at i = {}", i);
    }
}

#[test]
fn decode_slice() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        let mut buf = vec![0; expected.len()].into_boxed_slice();
        super::decode_slice(input, &mut buf, false).unwrap();
        assert_eq!(&*buf, expected, "Failed at i = {}", i);
    }
}
