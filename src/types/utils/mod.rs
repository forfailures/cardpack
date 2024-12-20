pub struct Bit;

impl Bit {
    const GUIDE: &'static str = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp";

    /// These utility methods come from `pkcore`, a library that is currently a work in progress.
    #[must_use]
    pub fn string(ckc: u32) -> String {
        let b = format!("{ckc:b}");
        // OK, let's take a moment to really stan on the rust std libraries. The fmt
        // [Fill/Alignment](https://doc.rust-lang.org/std/fmt/#fillalignment) is FIRE!
        let b = format!("{b:0>32}");
        let mut bit_string = String::with_capacity(34);

        for (i, c) in b.chars().enumerate() {
            bit_string.push(c);
            if i % 8 == 7 && i % 31 != 0 {
                bit_string.push(' ');
            }
        }
        bit_string
    }

    #[must_use]
    pub fn string_guided(ckc: u32) -> String {
        format!("{}\n{}", Bit::GUIDE, Bit::string(ckc))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__rank__tests {
    use super::*;

    #[test]
    fn string() {
        let ckc = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let expected = "00000000 00000000 00000000 00000000";
        assert_eq!(Bit::string(ckc), expected);

        let ckc = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let expected = "11111111 11111111 11111111 11111111";
        assert_eq!(Bit::string(ckc), expected);

        let ckc = 0b1010_1010_1010_1010_1010_1010_1010_1010;
        let expected = "10101010 10101010 10101010 10101010";
        assert_eq!(Bit::string(ckc), expected);
    }

    #[test]
    fn string_guided() {
        let ckc = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let expected = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n00000000 00000000 00000000 00000000";
        assert_eq!(Bit::string_guided(ckc), expected);

        let ckc = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let expected = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n11111111 11111111 11111111 11111111";
        assert_eq!(Bit::string_guided(ckc), expected);

        let ckc = 0b1010_1010_1010_1010_1010_1010_1010_1010;
        let expected = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n10101010 10101010 10101010 10101010";
        assert_eq!(Bit::string_guided(ckc), expected);
    }
}
