#[cfg(test)]
mod tests {
    use crate::utils::varint::parse_varint;

    #[test]
    fn test_7bit_positive_integer() {
        assert_eq!((127, 1), parse_varint(vec![127].as_slice()).unwrap());
        assert_eq!((0, 1), parse_varint(vec![0].as_slice()).unwrap());
        assert_eq!((42, 1), parse_varint(vec![42].as_slice()).unwrap());
    }

    #[test]
    fn test_14bit_positive_integer() {
        assert_eq!(
            (16383, 2),
            parse_varint(vec![0xBF, 0xFF].as_slice()).unwrap()
        );
        assert_eq!((0, 2), parse_varint(vec![0x80, 0].as_slice()).unwrap());
        assert_eq!((42, 2), parse_varint(vec![0x80, 42].as_slice()).unwrap());
    }

    #[test]
    fn test_21bit_positive_integer() {
        assert_eq!(
            (0x1FFFFF, 3),
            parse_varint(vec![0xDF, 0xFF, 0xFF].as_slice()).unwrap()
        );
        assert_eq!((0, 3), parse_varint(vec![0xC0, 0, 0].as_slice()).unwrap());
        assert_eq!((42, 3), parse_varint(vec![0xC0, 0, 42].as_slice()).unwrap());
    }

    #[test]
    fn test_28bit_positive_integer() {
        assert_eq!(
            (0xFFFFFFF, 4),
            parse_varint(vec![0xEF, 0xFF, 0xFF, 0xFF].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 4),
            parse_varint(vec![0xE0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 4),
            parse_varint(vec![0xE0, 0, 0, 42].as_slice()).unwrap()
        );
    }

    #[test]
    fn test_32bit_positive_integer() {
        assert_eq!(
            (0xFFFFFFFF, 5),
            parse_varint(vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()).unwrap()
        );
        assert_eq!(
            (0xFFFFFFFF, 5),
            parse_varint(vec![0xF2, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()).unwrap()
        );
        assert_eq!(
            (0xFFFFFFFF, 5),
            parse_varint(vec![0xF1, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()).unwrap()
        );
        assert_eq!(
            (0xFFFFFFFF, 5),
            parse_varint(vec![0xF0, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()).unwrap()
        );

        assert_eq!(
            (0, 5),
            parse_varint(vec![0xF3, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 5),
            parse_varint(vec![0xF2, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 5),
            parse_varint(vec![0xF1, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 5),
            parse_varint(vec![0xF0, 0, 0, 0, 0].as_slice()).unwrap()
        );

        assert_eq!(
            (42, 5),
            parse_varint(vec![0xF3, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 5),
            parse_varint(vec![0xF2, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 5),
            parse_varint(vec![0xF1, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 5),
            parse_varint(vec![0xF0, 0, 0, 0, 42].as_slice()).unwrap()
        );
    }

    #[test]
    fn test_64bit_positive_integer() {
        assert_eq!(
            (0xFFFFFFFFFFFFFFFF, 9),
            parse_varint(vec![0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .unwrap()
        );
        assert_eq!(
            (0xFFFFFFFFFFFFFFFF, 9),
            parse_varint(vec![0xF6, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .unwrap()
        );
        assert_eq!(
            (0xFFFFFFFFFFFFFFFF, 9),
            parse_varint(vec![0xF5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .unwrap()
        );
        assert_eq!(
            (0xFFFFFFFFFFFFFFFF, 9),
            parse_varint(vec![0xF4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .unwrap()
        );

        assert_eq!(
            (0, 9),
            parse_varint(vec![0xF7, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 9),
            parse_varint(vec![0xF6, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 9),
            parse_varint(vec![0xF5, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 9),
            parse_varint(vec![0xF4, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );

        assert_eq!(
            (42, 9),
            parse_varint(vec![0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 9),
            parse_varint(vec![0xF6, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 9),
            parse_varint(vec![0xF5, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (42, 9),
            parse_varint(vec![0xF4, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
    }

    #[test]
    fn test_negative_varint() {
        assert_eq!(
            (-0xFFFFFFFFFFFFFFFF, 10),
            parse_varint(
                vec![0xFB, 0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .unwrap()
        );
        assert_eq!(
            (-0xFFFFFFFFFFFFFFFF, 10),
            parse_varint(
                vec![0xFA, 0xF6, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .unwrap()
        );
        assert_eq!(
            (-0xFFFFFFFFFFFFFFFF, 10),
            parse_varint(
                vec![0xF9, 0xF5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .unwrap()
        );
        assert_eq!(
            (-0xFFFFFFFFFFFFFFFF, 10),
            parse_varint(
                vec![0xF8, 0xF4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .unwrap()
        );

        assert_eq!(
            (0, 10),
            parse_varint(vec![0xFB, 0xF4, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 10),
            parse_varint(vec![0xFA, 0xF5, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 10),
            parse_varint(vec![0xF9, 0xF6, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );
        assert_eq!(
            (0, 10),
            parse_varint(vec![0xF8, 0xF7, 0, 0, 0, 0, 0, 0, 0, 0].as_slice()).unwrap()
        );

        assert_eq!(
            (-42, 10),
            parse_varint(vec![0xFB, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (-42, 10),
            parse_varint(vec![0xFA, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (-42, 10),
            parse_varint(vec![0xF9, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
        assert_eq!(
            (-42, 10),
            parse_varint(vec![0xF8, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice()).unwrap()
        );
    }

    #[test]
    fn test_invalid_varint() {
        assert_eq!(true, parse_varint(vec![].as_slice()).is_err());
        assert_eq!(true, parse_varint(vec![0xBF].as_slice()).is_err());
        assert_eq!(true, parse_varint(vec![0xDF, 0xFF].as_slice()).is_err());
        assert_eq!(
            true,
            parse_varint(vec![0xEF, 0xFF, 0xFF].as_slice()).is_err()
        );
        assert_eq!(
            true,
            parse_varint(vec![0xF3, 0xFF, 0xFF, 0xFF].as_slice()).is_err()
        );
        assert_eq!(
            true,
            parse_varint(vec![0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()).is_err()
        );
        assert_eq!(
            true,
            parse_varint(vec![0xFB, 0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .is_err()
        );
    }
}
