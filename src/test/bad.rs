const TXT: &'static [&'static str] = &[include_str!("bad/abc.txt"),
                                       include_str!("bad/end_of_mid_stream_earlier.txt"),
                                       include_str!("bad/end_of_stream_begins_stream.txt"),
                                       include_str!("bad/end_of_stream_mid_stream.txt"),
                                       include_str!("bad/eos_then_junk.txt"),
                                       include_str!("bad/junk_on_end.txt"),
                                       include_str!("bad/line_break.txt"),
                                       include_str!("bad/rogue_end_of_stream_char.txt"),
                                       include_str!("bad/two_ends_of_stream.txt")];

#[test]
fn decode() {
    for enctx in TXT {
        assert!(super::decode(enctx).is_err());
    }
}
