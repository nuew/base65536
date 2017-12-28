use test_crate::test::Bencher;

const BIN: &[u8] = include_bytes!("common/data/pairs/sample-files/everyByte.bin");
const TXT: &str = include_str!("common/data/pairs/sample-files/everyByte.txt");
const TXT_IGNORE: &str = include_str!(
    "common/data/ignoreGarbage/randomAlphanumericInterference.\
     txt"
);

#[bench]
fn decode(b: &mut Bencher) {
    let _ = super::decode(TXT, false); // spin up B2S (lazy_static)

    b.iter(|| super::decode(TXT, false));
}

#[bench]
fn decode_ignore_garbage_with_clean_input(b: &mut Bencher) {
    let _ = super::decode(TXT, false); // spin up B2S (lazy_static)

    b.iter(|| super::decode(TXT, true));
}

#[bench]
fn decode_ignore_garbage_with_garbage_input(b: &mut Bencher) {
    let _ = super::decode(TXT, false); // spin up B2S (lazy_static)

    b.iter(|| super::decode(TXT_IGNORE, true));
}

#[bench]
fn decode_config_buf_naive(b: &mut Bencher) {
    let _ = super::decode(TXT, false); // spin up B2S (lazy_static)

    b.iter(|| {
        let mut buf = Vec::new();
        super::decode_buf(TXT, &mut buf, false)
    });
}

#[bench]
fn decode_config_buf_smart(b: &mut Bencher) {
    let _ = super::decode(TXT, false); // spin up B2S (lazy_static)

    b.iter(|| {
        let mut buf = Vec::with_capacity(TXT.len());
        super::decode_buf(TXT, &mut buf, false)
    });
}

#[bench]
fn decode_slice(b: &mut Bencher) {
    let _ = super::decode(TXT, false); // spin up B2S (lazy_static)

    // the idea is that this has a static size and is stack allocated
    // but slice.len() isn't a const function even on const arrays
    let mut buf = vec![0; TXT.len()].into_boxed_slice();
    b.iter(|| super::decode_slice(TXT, &mut buf, false));
}

#[bench]
fn encode(b: &mut Bencher) {
    b.iter(|| super::encode(BIN, None));
}

#[bench]
fn encode_config_buf_naive(b: &mut Bencher) {
    b.iter(|| {
        let mut string = String::new();
        super::encode_buf(BIN, &mut string, None)
    });
}

#[bench]
fn encode_config_buf_smart(b: &mut Bencher) {
    b.iter(|| {
        let mut string = String::with_capacity(BIN.len() * 2);
        super::encode_buf(BIN, &mut string, None);
    });
}

macro_rules! encode_wrap_n{
    ( $n:expr, $encode:ident ) => {
        #[bench]
        fn $encode(b: &mut Bencher) {
            b.iter(|| super::encode(BIN, $n));
        }
    };
}

encode_wrap_n!(1, encode_wrap_1);
encode_wrap_n!(2, encode_wrap_2);
encode_wrap_n!(4, encode_wrap_4);
encode_wrap_n!(5, encode_wrap_5);
encode_wrap_n!(76, encode_wrap_76);
encode_wrap_n!(140, encode_wrap_140);
encode_wrap_n!(256, encode_wrap_256);
encode_wrap_n!(1000, encode_wrap_1000);
