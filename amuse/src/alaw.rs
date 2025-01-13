use crate::traits::Compander;

#[derive(Debug, Clone, Copy, Default)]
pub struct ALaw;

impl Compander<i16, u8> for ALaw {
    fn compress(linear: i16) -> u8 {
        let is_negative = linear.is_negative();

        #[allow(
            clippy::cast_sign_loss,
            reason = "We guarantee the castee is positive by NOT-ing negative values (i.e. flipping the sign bit)."
        )]
        let ix = (if is_negative { !linear } else { linear }) as u16 >> 4;
        let sign_bit = u8::from(!is_negative) << 7;

        #[allow(
            clippy::cast_possible_truncation,
            reason = "There can only ever be 16 leading zeroes in u16, which fits in a u8."
        )]
        let leading_zeroes = ix.leading_zeros() as u8;

        let magnitude = if leading_zeroes < 12 {
            let mantissa = ((ix >> (11 - leading_zeroes)) & 0b1111) as u8;
            let exponent = (12 - leading_zeroes) << 4;

            mantissa | exponent
        } else {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "`ix` always has at least 12 leading zeroes due to conditional."
            )]
            let byte = ix as u8;

            byte
        };

        (magnitude | sign_bit) ^ 0b0101_0101
    }

    fn expand(log: u8) -> i16 {
        let sign = 1 - 2 * i16::from(u8::from(log < 0b1000_0000));
        let ix = (log ^ 0b0101_0101) & 0b0111_1111;

        let exponent = ix >> 4;
        let mantissa = {
            let low_nibble = ix & (0b1111);
            let nonzero_exponent_marker_bit = u8::from(exponent > 0) << 4;

            let base = (i16::from(low_nibble | nonzero_exponent_marker_bit) << 4) | 0b1000;
            let offset = u8::from(exponent > 1) * exponent.saturating_sub(1);

            base << offset
        };

        sign * mantissa
    }
}
