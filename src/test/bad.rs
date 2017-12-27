const TXT: &'static [&'static str] = &[
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
