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
        let mut ix = (if is_negative { !linear } else { linear }) as u16 >> 4;

        if ix > 0b1111 {
            let mut iexp = 1;
            while ix > 0b1_1111 {
                ix >>= 1;
                iexp += 1;
            }
            ix -= 16;

            ix += iexp << 4;
        }

        let sign_bit = u8::from(!is_negative) << 7;

        #[allow(
            clippy::cast_possible_truncation,
            reason = "ix takes up 8 bits after above logic"
        )]
        let out = (sign_bit | ix as u8) ^ 0b0101_0101;

        out
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
