use super::*;

mod decode {
    use super::decode;

    #[test]
    fn hello_world() {
        let input = "\u{9A68}\u{A36C}\u{556F}\u{12077}\u{A372}\u{1564}";
        let expected = b"hello world";

        assert_eq!(decode(input).unwrap(), expected);
    }

    #[test]
    fn empty() {
        let input = "";
        let expected = b"";

        assert_eq!(decode(input).unwrap(), expected)
    }
}

mod encode {
    use super::encode;

    #[test]
    fn hello_world() {
        let input = "hello world";
        let expected = "\u{9A68}\u{A36C}\u{556F}\u{12077}\u{A372}\u{1564}";

        assert_eq!(encode(input), expected);
    }

    #[test]
    fn empty() {
        let input = "";
        let expected = "";

        assert_eq!(encode(input), expected)
    }
}

mod round_trip {
    use super::{decode, encode};

    #[test]
    fn hello_world() {
        let input = b"hello world";

        assert_eq!(decode(&encode(input)).unwrap(), input);
    }

    #[test]
    fn empty() {
        let input = b"";

        assert_eq!(decode(&encode(input)).unwrap(), input)
    }
}
