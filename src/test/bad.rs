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

const TXT: &[&str] = &[
    include_str!("common/data/bad/lineBreak.txt"),
    include_str!("common/data/bad/endOfStreamBeginsStream.txt"),
    include_str!("common/data/bad/endOfStreamMidStream.txt"),
    include_str!("common/data/bad/twoEndsOfStream.txt"),
    include_str!("common/data/bad/eosThenJunk.txt"),
    include_str!("common/data/bad/junkOnEnd.txt"),
    include_str!("common/data/bad/abc.txt"),
    include_str!(
        "common/data/bad/endOfStreamMidStreamEarlier.\
         txt"
    ),
    include_str!("common/data/bad/rogueEndOfStreamChar.txt"),
];

#[test]
fn decode() {
    for enctx in TXT {
        assert!(super::decode(enctx, false).is_err());
    }
}

#[test]
fn decode_buf() {
    for enctx in TXT {
        let mut buf = Vec::new();
        assert!(super::decode_buf(enctx, &mut buf, false).is_err());
    }
}

#[test]
fn decode_slice() {
    for enctx in TXT {
        let mut buf = vec![0; enctx.len()].into_boxed_slice();
        assert!(super::decode_slice(enctx, &mut buf, false).is_err());
    }
}
