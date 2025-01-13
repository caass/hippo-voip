use crate::traits::Compander;

#[derive(Debug, Clone, Copy, Default)]
pub struct ULaw;

impl Compander<i16, u8> for ULaw {
    fn compress(linear: i16) -> u8 {
        let is_negative = linear.is_negative();

        #[allow(
            clippy::cast_sign_loss,
            reason = "We guarantee the castee is positive by NOT-ing negative values (i.e. flipping the sign bit)."
        )]
        let offset = u16::min(
            ((if is_negative { !linear } else { linear } as u16) >> 2) + 0x21,
            0x1FFF,
        );

        #[allow(
            clippy::cast_possible_truncation,
            reason = "There can only ever be 16 leading zeroes in u16, which fits in a u8."
        )]
        let segment = 11 - (offset.leading_zeros() as u8);

        #[allow(
            clippy::cast_possible_truncation,
            reason = "`segment` increases with `offset` such that `offset >> segment <= 255` always holds."
        )]
        let low_nibble = 0b1111 - (((offset >> segment) as u8) & 0b1111);
        let high_nibble = (8 - segment) << 4;
        let sign_bit = u8::from(!is_negative) << 7;

        high_nibble | low_nibble | sign_bit
    }

    fn expand(log: u8) -> i16 {
        let sign = 2 * i16::from(log >> 7) - 1;

        let inverted = !log;
        let exponent = (inverted >> 4) & 0b0111;
        let mantissa = inverted & 0b1111;
        let step = 4i16 << (exponent + 1);

        sign * ((0b1000_0000 << exponent) + (step * i16::from(mantissa)) + (step / 2) - (4 * 0x21))
    }
}
