const BIN: &'static [&'static [u8]] = &[include_bytes!("ignore_garbage/abc.bin"),
                                        include_bytes!("ignore_garbage/continuation_at_end.bin"),
                                        include_bytes!("ignore_garbage/line_break.bin"),
                                        include_bytes!("ignore_garbage/line_breaks.bin"),
                                        include_bytes!("ignore_garbage/quoted.bin"),
                                        include_bytes!("ignore_garbage/random_alphanumeric_interference.\
                                                        bin"),
                                        include_bytes!("ignore_garbage/space_after.bin"),
                                        include_bytes!("ignore_garbage/space_before.bin"),
                                        include_bytes!("ignore_garbage/spaces_everywhere.bin")];
const TXT: &'static [&'static str] = &[include_str!("ignore_garbage/abc.txt"),
                                       include_str!("ignore_garbage/continuation_at_end.txt"),
                                       include_str!("ignore_garbage/line_break.txt"),
                                       include_str!("ignore_garbage/line_breaks.txt"),
                                       include_str!("ignore_garbage/quoted.txt"),
                                       include_str!("ignore_garbage/random_alphanumeric_interference.\
                                                     txt"),
                                       include_str!("ignore_garbage/space_after.txt"),
                                       include_str!("ignore_garbage/space_before.txt"),
                                       include_str!("ignore_garbage/spaces_everywhere.txt")];

#[test]
fn sanity() {
    assert_eq!(BIN.len(), TXT.len());
}

#[test]
fn decode() {
    let cfg = super::Config::new().ignore_garbage(true);

    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        assert_eq!(super::decode_config(input, cfg).unwrap(),
                   expected,
                   "Failed at i = {}",
                   i);
    }
}
