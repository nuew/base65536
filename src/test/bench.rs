use test_crate::test::Bencher;

const BIN: &'static [u8] = include_bytes!("common/data/pairs/sample-files/everyByte.bin");
const TXT: &'static str = include_str!("common/data/pairs/sample-files/everyByte.txt");
const TXT_IGNORE: &'static str = include_str!("common/data/ignoreGarbage/randomAlphanumericInterference.\
                                               txt");

#[bench]
fn decode(b: &mut Bencher) {
    let _ = super::decode(TXT); // spin up B2S (lazy_static)

    b.iter(|| super::decode(TXT));
}

#[bench]
fn decode_config(b: &mut Bencher) {
    let config = super::Config::default();
    let _ = super::decode(TXT); // spin up B2S (lazy_static)

    b.iter(|| super::decode_config(TXT, config));
}

#[bench]
fn decode_ignore_garbage_with_clean_input(b: &mut Bencher) {
    let config = super::Config::new().ignore_garbage(true);
    let _ = super::decode(TXT); // spin up B2S (lazy_static)

    b.iter(|| super::decode_config(TXT, config));
}

#[bench]
fn decode_ignore_garbage_with_garbage_input(b: &mut Bencher) {
    let config = super::Config::new().ignore_garbage(true);
    let _ = super::decode(TXT); // spin up B2S (lazy_static)

    b.iter(|| super::decode_config(TXT_IGNORE, config));
}

#[bench]
fn decode_config_buf_naive(b: &mut Bencher) {
    let config = super::Config::default();
    let _ = super::decode(TXT); // spin up B2S (lazy_static)

    b.iter(|| {
        let mut buf = Vec::new();
        super::decode_config_buf(TXT, config, &mut buf)
    });
}

#[bench]
fn decode_config_buf_smart(b: &mut Bencher) {
    let config = super::Config::default();
    let _ = super::decode(TXT); // spin up B2S (lazy_static)

    b.iter(|| {
        let mut buf = Vec::with_capacity(TXT.len());
        super::decode_config_buf(TXT, config, &mut buf)
    });
}

#[bench]
fn encode(b: &mut Bencher) {
    b.iter(|| super::encode(BIN));
}

#[bench]
fn encode_config(b: &mut Bencher) {
    let config = super::Config::default();

    b.iter(|| super::encode_config(BIN, config));
}

#[bench]
fn encode_config_buf_naive(b: &mut Bencher) {
    let config = super::Config::default();

    b.iter(|| {
        let mut string = String::new();
        super::encode_config_buf(BIN, config, &mut string)
    });
}

#[bench]
fn encode_config_buf_smart(b: &mut Bencher) {
    let config = super::Config::default();

    b.iter(|| {
        let mut string = String::with_capacity(BIN.len() * 2);
        super::encode_config_buf(BIN, config, &mut string);
    });
}

macro_rules! encode_wrap_n{
    ( $n:expr, $encode:ident ) => {
        #[bench]
        fn $encode(b: &mut Bencher) {
            let config = super::Config::new().wrap(Some(($n, "\n")));

            b.iter(|| super::encode_config(BIN, config));
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
