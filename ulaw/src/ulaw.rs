use crate::traits::Compander;

#[derive(Debug, Clone, Copy, Default)]
pub struct ULaw;

impl Compander<i16, u8> for ULaw {
    fn compress(linear: i16) -> u8 {
        let is_negative = linear.is_negative();

        // Compute the interval number.
        //
        // 1. Use 1's complement to make all values positive.
        // 2. Right-shift by 2 to right-align the previously left-aligned 14-bit value.
        // 3. Offset by the difference between mu-law and A-law (33).
        // 4. Clamp to i14::MAX.
        #[allow(
            clippy::cast_sign_loss,
            reason = "We guarantee the castee is positive by NOT-ing negative values (i.e. flipping the sign bit)."
        )]
        let absno = u16::min(
            ((if is_negative { !linear } else { linear } as u16) >> 2) + 0x21,
            0x1FFF,
        );

        // N.B.: 0x20 (0b10_0000) < `absno` < 0x2000 (0b10_0000_0000_0000) due to offsetting and clamping.

        // Compute the segment number.
        //
        // Since `absno` is guaranteed to be between 0x20 and 0x2000, there is guaranteed to be between
        // 3 and 10 leading zeroes. Since there's 8 segments (and they're 1-indexed), we can subtract
        // the number of leading zeroes from 11 to compute the segment number without looping and right-shifting
        // (which is what the reference implementation does).

        #[allow(
            clippy::cast_possible_truncation,
            reason = "There can only ever be 16 leading zeroes in u16, which fits in a u8."
        )]
        let segno = 11u8 - (absno.leading_zeros() as u8);

        // N.B.: 0 < segno < 9

        // compute the mu-law compressed value 0bA_BBB_CCCC where
        //  - A is the sign bit (1 for positive numbers, 0 for negative)
        //  - BBB is the segment number
        //  - CCCC is the interval number

        #[allow(
            clippy::cast_possible_truncation,
            reason = "`segno` increases with `absno` such that `absno >> segno` will never take more than 8 bits."
        )]
        let compressed = ((8u8 - segno) << 4)
            | (0b1111u8 - (((absno >> segno) as u8) & 0b1111))
            | (u8::from(!is_negative) << 7);

        compressed
    }

    fn expand(log: u8) -> i16 {
        let sign = if log < 0b1000_0000 { -1i16 } else { 1 };

        let mut mantissa = !log;
        let exponent = (mantissa >> 4) & 0b111;
        mantissa &= 0b1111;

        let step = 4i16 << (exponent + 1);

        sign * ((0b1000_0000 << exponent) + (step * i16::from(mantissa)) + (step / 2) - (4 * 33))
    }
}
