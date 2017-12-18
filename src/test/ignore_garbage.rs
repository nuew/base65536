const BIN: &'static [&'static [u8]] =
    &[include_bytes!("common/data/ignoreGarbage/abc.bin"),
      include_bytes!("common/data/ignoreGarbage/continuationAtEnd.bin"),
      include_bytes!("common/data/ignoreGarbage/lineBreak.bin"),
      include_bytes!("common/data/ignoreGarbage/lineBreaks.bin"),
      include_bytes!("common/data/ignoreGarbage/quoted.bin"),
      include_bytes!("common/data/ignoreGarbage/randomAlphanumericInterference.bin"),
      include_bytes!("common/data/ignoreGarbage/spaceAfter.bin"),
      include_bytes!("common/data/ignoreGarbage/spaceBefore.bin"),
      include_bytes!("common/data/ignoreGarbage/spacesEverywhere.bin")];

const TXT: &'static [&'static str] =
    &[include_str!("common/data/ignoreGarbage/abc.txt"),
      include_str!("common/data/ignoreGarbage/continuationAtEnd.txt"),
      include_str!("common/data/ignoreGarbage/lineBreak.txt"),
      include_str!("common/data/ignoreGarbage/lineBreaks.txt"),
      include_str!("common/data/ignoreGarbage/quoted.txt"),
      include_str!("common/data/ignoreGarbage/randomAlphanumericInterference.txt"),
      include_str!("common/data/ignoreGarbage/spaceAfter.txt"),
      include_str!("common/data/ignoreGarbage/spaceBefore.txt"),
      include_str!("common/data/ignoreGarbage/spacesEverywhere.txt")];

#[test]
fn sanity() {
    assert_eq!(BIN.len(), TXT.len());
}

#[test]
fn decode() {
    for i in 0..TXT.len() {
        let input = TXT[i];
        let expected = BIN[i];

        assert_eq!(super::decode(input, true).unwrap(),
                   expected,
                   "Failed at i = {}",
                   i);
    }
}
