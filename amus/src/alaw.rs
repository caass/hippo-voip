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

        let leading_zeroes = ix.leading_zeros() as u8;
        ((if leading_zeroes < 12 {
            ((ix >> (11 - leading_zeroes)) & 0b1111) as u8 | ((12 - leading_zeroes) << 4)
        } else {
            ix as u8
        }) | (u8::from(!is_negative) << 7))
            ^ 0b0101_0101
    }

    fn expand(log: u8) -> i16 {
        let mut ix = i16::from(log) ^ 0b0101_0101;

        ix &= 0x007F;
        let iexp = ix >> 4;
        let mut mant = ix & (0x000F);
        if iexp > 0 {
            mant += 16;
        }

        mant = (mant << 4) + (0x0008);

        if iexp > 1 {
            mant <<= iexp - 1;
        }

        if log > 127 {
            mant
        } else {
            -mant
        }
    }
}
