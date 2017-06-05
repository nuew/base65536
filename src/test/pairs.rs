const BIN: &'static [&'static [u8]] = &[include_bytes!("pairs/empty.bin"),
                                        include_bytes!("pairs/every_byte.bin"),
                                        include_bytes!("pairs/every_pair_of_bytes.bin"),
                                        include_bytes!("pairs/first_defect.bin"),
                                        include_bytes!("pairs/hatetris_wr.bin"),
                                        include_bytes!("pairs/hatetris_wr_rle.bin"),
                                        include_bytes!("pairs/hatetris_wr_rle_2.bin"),
                                        include_bytes!("pairs/hello_world.bin"),
                                        include_bytes!("pairs/lena_std.tif.bin")];

const TXT: &'static [&'static str] = &[include_str!("pairs/empty.txt"),
                                       include_str!("pairs/every_byte.txt"),
                                       include_str!("pairs/every_pair_of_bytes.txt"),
                                       include_str!("pairs/first_defect.txt"),
                                       include_str!("pairs/hatetris_wr.txt"),
                                       include_str!("pairs/hatetris_wr_rle.txt"),
                                       include_str!("pairs/hatetris_wr_rle_2.txt"),
                                       include_str!("pairs/hello_world.txt"),
                                       include_str!("pairs/lena_std.tif.txt")];

#[test]
fn sanity() {
    assert_eq!(BIN.len(), TXT.len());
}

#[test]
fn encode() {
    for i in 0..BIN.len() {
        let input = BIN[i];
        let expected = TXT[i];

        assert_eq!(super::encode(input), expected, "Failed at i = {}", i);
    }
}

#[test]
fn decode() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        assert_eq!(super::decode(input).unwrap(),
                   expected,
                   "Failed at i = {}",
                   i);
    }
}
