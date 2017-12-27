const BIN: &'static [&'static [u8]] = &[
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

const TXT: &'static [&'static str] = &[
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
