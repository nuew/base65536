const BIN_1: &[&[u8]] = &[
    include_bytes!("common/data/wrap/1/empty.bin"),
    include_bytes!("common/data/wrap/1/hatetris-wr.bin"),
];
const TXT_1: &[&str] = &[
    include_str!("common/data/wrap/1/empty.txt"),
    include_str!("common/data/wrap/1/hatetris-wr.txt"),
];

const BIN_2: &[&[u8]] = &[
    include_bytes!("common/data/wrap/2/empty.bin"),
    include_bytes!("common/data/wrap/2/hatetris-wr-rle.bin"),
];
const TXT_2: &[&str] = &[
    include_str!("common/data/wrap/2/empty.txt"),
    include_str!("common/data/wrap/2/hatetris-wr-rle.txt"),
];

const BIN_4: &[&[u8]] = &[
    include_bytes!("common/data/wrap/4/empty.bin"),
    include_bytes!(
        "common/data/wrap/4/hatetris-wr-rle2.\
         bin"
    ),
];
const TXT_4: &[&str] = &[
    include_str!("common/data/wrap/4/empty.txt"),
    include_str!("common/data/wrap/4/hatetris-wr-rle2.txt"),
];

const BIN_5: &[&[u8]] = &[
    include_bytes!("common/data/wrap/5/empty.bin"),
    include_bytes!("common/data/wrap/5/demo.bin"),
];
const TXT_5: &[&str] = &[
    include_str!("common/data/wrap/5/empty.txt"),
    include_str!("common/data/wrap/5/demo.txt"),
];

const BIN_76: &[&[u8]] = &[
    include_bytes!("common/data/wrap/76/empty.bin"),
    include_bytes!("common/data/wrap/76/everyByte.bin"),
];
const TXT_76: &[&str] = &[
    include_str!("common/data/wrap/76/empty.txt"),
    include_str!("common/data/wrap/76/everyByte.txt"),
];

const BIN_140: &[&[u8]] = &[
    include_bytes!("common/data/wrap/140/empty.bin"),
    include_bytes!(
        "common/data/wrap/140/lena_std.tif.\
         bin"
    ),
];
const TXT_140: &[&str] = &[
    include_str!("common/data/wrap/140/empty.txt"),
    include_str!("common/data/wrap/140/lena_std.tif.txt"),
];

const BIN_256: &[&[u8]] = &[
    include_bytes!("common/data/wrap/256/empty.bin"),
    include_bytes!(
        "common/data/wrap/256/everyPairOfBytes.\
         bin"
    ),
];
const TXT_256: &[&str] = &[
    include_str!("common/data/wrap/256/empty.txt"),
    include_str!(
        "common/data/wrap/256/everyPairOfBytes.\
         txt"
    ),
];

const BIN_1000: &[&[u8]] = &[
    include_bytes!("common/data/wrap/1000/empty.bin"),
    include_bytes!(
        "common/data/wrap/1000/everyPairOfBytes.\
         bin"
    ),
];
const TXT_1000: &[&str] = &[
    include_str!("common/data/wrap/1000/empty.txt"),
    include_str!(
        "common/data/wrap/1000/everyPairOfBytes.\
         txt"
    ),
];

macro_rules! wrap_n {
    ( $n:expr, $sanity:ident, $encode:ident, $encode_buf:ident, $bin:ident, $txt:ident ) => {
        #[test]
        fn $sanity() {
            assert_eq!($bin.len(), $txt.len());
        }

        #[test]
        fn $encode() {
            for i in 0..$bin.len() {
                let input = $bin[i];
                let expected = $txt[i];

                assert_eq!(super::encode(input, $n),
                           expected,
                           "Failed at i = {}",
                           i);
            }
        }

        #[test]
        fn $encode_buf() {
            for i in 0..$bin.len() {
                let input = $bin[i];
                let expected = $txt[i];

                let mut buf = String::new();
                super::encode_buf(input, &mut buf, $n);
                assert_eq!(buf, expected, "Failed at i = {}", i);
            }
        }
    };
}

wrap_n!(1, sanity_1, encode_1, encode_buf_1, BIN_1, TXT_1);
wrap_n!(2, sanity_2, encode_2, encode_buf_2, BIN_2, TXT_2);
wrap_n!(4, sanity_4, encode_4, encode_buf_3, BIN_4, TXT_4);
wrap_n!(5, sanity_5, encode_5, encode_buf_5, BIN_5, TXT_5);
wrap_n!(76, sanity_76, encode_76, encode_buf_76, BIN_76, TXT_76);
wrap_n!(
    140,
    sanity_140,
    encode_140,
    encode_buf_140,
    BIN_140,
    TXT_140
);
wrap_n!(
    256,
    sanity_256,
    encode_256,
    encode_buf_256,
    BIN_256,
    TXT_256
);
wrap_n!(
    1000,
    sanity_1000,
    encode_1000,
    encode_buf_1000,
    BIN_1000,
    TXT_1000
);
