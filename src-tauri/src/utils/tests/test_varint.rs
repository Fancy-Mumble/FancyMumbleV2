#[cfg(test)]
mod tests {
    use crate::utils::varint::Builder;

    #[test]
    fn test_7bit_positive_integer() {
        assert_eq!(
            (127, 1),
            Builder::from(vec![127].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 1),
            Builder::from(vec![0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 1),
            Builder::from(vec![42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_14bit_positive_integer() {
        assert_eq!(
            (16383, 2),
            Builder::from(vec![0xBF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 2),
            Builder::from(vec![0x80, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 2),
            Builder::from(vec![0x80, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_21bit_positive_integer() {
        assert_eq!(
            (0x001F_FFFF, 3),
            Builder::from(vec![0xDF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 3),
            Builder::from(vec![0xC0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 3),
            Builder::from(vec![0xC0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_28bit_positive_integer() {
        assert_eq!(
            (0x0FFF_FFFF, 4),
            Builder::from(vec![0xEF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 4),
            Builder::from(vec![0xE0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 4),
            Builder::from(vec![0xE0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_32bit_positive_integer() {
        assert_eq!(
            (0xFFFF_FFFF, 5),
            Builder::from(vec![0xF3, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0xFFFF_FFFF, 5),
            Builder::from(vec![0xF2, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0xFFFF_FFFF, 5),
            Builder::from(vec![0xF1, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0xFFFF_FFFF, 5),
            Builder::from(vec![0xF0, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );

        assert_eq!(
            (0, 5),
            Builder::from(vec![0xF3, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 5),
            Builder::from(vec![0xF2, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 5),
            Builder::from(vec![0xF1, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 5),
            Builder::from(vec![0xF0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );

        assert_eq!(
            (42, 5),
            Builder::from(vec![0xF3, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 5),
            Builder::from(vec![0xF2, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 5),
            Builder::from(vec![0xF1, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 5),
            Builder::from(vec![0xF0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_64bit_positive_integer() {
        assert_eq!(
            (0xFFFF_FFFF_FFFF_FFFF, 9),
            Builder::from(vec![0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0xFFFF_FFFF_FFFF_FFFF, 9),
            Builder::from(vec![0xF6, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0xFFFF_FFFF_FFFF_FFFF, 9),
            Builder::from(vec![0xF5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0xFFFF_FFFF_FFFF_FFFF, 9),
            Builder::from(vec![0xF4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );

        assert_eq!(
            (0, 9),
            Builder::from(vec![0xF7, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 9),
            Builder::from(vec![0xF6, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 9),
            Builder::from(vec![0xF5, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 9),
            Builder::from(vec![0xF4, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );

        assert_eq!(
            (42, 9),
            Builder::from(vec![0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 9),
            Builder::from(vec![0xF6, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 9),
            Builder::from(vec![0xF5, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (42, 9),
            Builder::from(vec![0xF4, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_negative_varint() {
        assert_eq!(
            (-0xFFFF_FFFF_FFFF_FFFF, 10),
            Builder::from(
                vec![0xFB, 0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .build()
            .unwrap()
            .parsed_pair()
        );
        assert_eq!(
            (-0xFFFF_FFFF_FFFF_FFFF, 10),
            Builder::from(
                vec![0xFA, 0xF6, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .build()
            .unwrap()
            .parsed_pair()
        );
        assert_eq!(
            (-0xFFFF_FFFF_FFFF_FFFF, 10),
            Builder::from(
                vec![0xF9, 0xF5, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .build()
            .unwrap()
            .parsed_pair()
        );
        assert_eq!(
            (-0xFFFF_FFFF_FFFF_FFFF, 10),
            Builder::from(
                vec![0xF8, 0xF4, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice()
            )
            .build()
            .unwrap()
            .parsed_pair()
        );

        assert_eq!(
            (0, 10),
            Builder::from(vec![0xFB, 0xF4, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 10),
            Builder::from(vec![0xFA, 0xF5, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 10),
            Builder::from(vec![0xF9, 0xF6, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (0, 10),
            Builder::from(vec![0xF8, 0xF7, 0, 0, 0, 0, 0, 0, 0, 0].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );

        assert_eq!(
            (-42, 10),
            Builder::from(vec![0xFB, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (-42, 10),
            Builder::from(vec![0xFA, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (-42, 10),
            Builder::from(vec![0xF9, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
        assert_eq!(
            (-42, 10),
            Builder::from(vec![0xF8, 0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
                .build()
                .unwrap()
                .parsed_pair()
        );
    }

    #[test]
    fn test_invalid_varint() {
        assert_eq!(true, Builder::from(vec![].as_slice()).build().is_err());
        assert_eq!(true, Builder::from(vec![0xBF].as_slice()).build().is_err());
        assert_eq!(
            true,
            Builder::from(vec![0xDF, 0xFF].as_slice()).build().is_err()
        );
        assert_eq!(
            true,
            Builder::from(vec![0xEF, 0xFF, 0xFF].as_slice())
                .build()
                .is_err()
        );
        assert_eq!(
            true,
            Builder::from(vec![0xF3, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .is_err()
        );
        assert_eq!(
            true,
            Builder::from(vec![0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .is_err()
        );
        assert_eq!(
            true,
            Builder::from(vec![0xFB, 0xF7, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice())
                .build()
                .is_err()
        );
    }

    #[test]
    fn test_varint_builder_parsing() {
        let builder_varint = Builder::from(vec![0xF7, 0, 0, 0, 0, 0, 0, 0, 42].as_slice())
            .build()
            .unwrap();

        assert_eq!(42, builder_varint.parsed_value);
        assert_eq!(9, builder_varint.parsed_bytes);
    }
}
