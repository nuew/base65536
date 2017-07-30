const BIN_1: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/1/empty.bin"),
                                          include_bytes!("common/data/wrap/1/hatetris-wr.bin")];
const TXT_1: &'static [&'static str] = &[include_str!("common/data/wrap/1/empty.txt"),
                                         include_str!("common/data/wrap/1/hatetris-wr.txt")];

const BIN_2: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/2/empty.bin"),
                                          include_bytes!("common/data/wrap/2/hatetris-wr-rle.bin")];
const TXT_2: &'static [&'static str] = &[include_str!("common/data/wrap/2/empty.txt"),
                                         include_str!("common/data/wrap/2/hatetris-wr-rle.txt")];

const BIN_4: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/4/empty.bin"),
                                          include_bytes!("common/data/wrap/4/hatetris-wr-rle2.\
                                                          bin")];
const TXT_4: &'static [&'static str] = &[include_str!("common/data/wrap/4/empty.txt"),
                                         include_str!("common/data/wrap/4/hatetris-wr-rle2.txt")];

const BIN_5: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/5/empty.bin"),
                                          include_bytes!("common/data/wrap/5/demo.bin")];
const TXT_5: &'static [&'static str] = &[include_str!("common/data/wrap/5/empty.txt"),
                                         include_str!("common/data/wrap/5/demo.txt")];

const BIN_76: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/76/empty.bin"),
                                           include_bytes!("common/data/wrap/76/everyByte.bin")];
const TXT_76: &'static [&'static str] = &[include_str!("common/data/wrap/76/empty.txt"),
                                          include_str!("common/data/wrap/76/everyByte.txt")];

const BIN_140: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/140/empty.bin"),
                                            include_bytes!("common/data/wrap/140/lena_std.tif.\
                                                            bin")];
const TXT_140: &'static [&'static str] = &[include_str!("common/data/wrap/140/empty.txt"),
                                           include_str!("common/data/wrap/140/lena_std.tif.txt")];

const BIN_256: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/256/empty.bin"),
                                            include_bytes!("common/data/wrap/256/everyPairOfBytes.\
                                                            bin")];
const TXT_256: &'static [&'static str] = &[include_str!("common/data/wrap/256/empty.txt"),
                                           include_str!("common/data/wrap/256/everyPairOfBytes.\
                                                         txt")];

const BIN_1000: &'static [&'static [u8]] = &[include_bytes!("common/data/wrap/1000/empty.bin"),
                                             include_bytes!("common/data/wrap/1000/everyPairOfBytes.\
                                                             bin")];
const TXT_1000: &'static [&'static str] = &[include_str!("common/data/wrap/1000/empty.txt"),
                                            include_str!("common/data/wrap/1000/everyPairOfBytes.\
                                                          txt")];

macro_rules! wrap_n {
    ( $n:expr, $sanity:ident, $encode:ident, $bin:ident, $txt:ident ) => {
        #[test]
        fn $sanity() {
            assert_eq!($bin.len(), $txt.len());
        }

        #[test]
        fn $encode() {
            let cfg = super::Config::new().wrap(Some(($n, "\n")));

            for i in 0..$bin.len() {
                let input = $bin[i];
                let expected = $txt[i];

                assert_eq!(super::encode_config(input, cfg),
                           expected,
                           "Failed at i = {}",
                           i);
            }
        }
    };
}

wrap_n!(1, sanity_1, encode_1, BIN_1, TXT_1);
wrap_n!(2, sanity_2, encode_2, BIN_2, TXT_2);
wrap_n!(4, sanity_4, encode_4, BIN_4, TXT_4);
wrap_n!(5, sanity_5, encode_5, BIN_5, TXT_5);
wrap_n!(76, sanity_76, encode_76, BIN_76, TXT_76);
wrap_n!(140, sanity_140, encode_140, BIN_140, TXT_140);
wrap_n!(256, sanity_256, encode_256, BIN_256, TXT_256);
wrap_n!(1000, sanity_1000, encode_1000, BIN_1000, TXT_1000);