//! # amuse
//!
//! Pure-rust implementations of the [μ-law][1] and [A-law][2] companding algorithms as specified in
//! [ITU-T Recommendation G.711][3].
//!
//! The algorithms in `amuse` are drop-in compatible with the implementations in the
//! [ITU-T Software Tool Library (G.191)][4] ([here][5]), but `amuse` doesn't link against those implementations
//! unless the `g191` feature is enabled.
//!
//! In general, you'll want to use `amuse` by bringing the various compansion traits into scope with a prelude import.
//! These traits provide access to the [`.expand()`][6], [`.compress()`][7], [`.expand_buf()`][8], and
//! [`.compress_buf()`][9] methods. These methods are generic over either algorithm ([`ALaw`][10] or [`ULaw`][11]),
//! so you can use the turbofish operator to specify the desired algorithm:
//!
//! ```
//! use amuse::prelude::*;
//!
//! const ENCODED: [u8; 32] = *b"This string is ulaw encoded data";
//!
//! let decoded = ENCODED.expand::<ULaw>();
//! assert_eq!(decoded, [
//!     -748, -244, -228, -96, -7932, -96, -88, -104, -228, -148, -260,
//!     -7932, -228, -96, -7932, -80, -180, -356, -64, -7932, -292, -148,
//!     -324, -132, -308, -292, -308, -7932, -308, -356, -88, -356
//! ]);
//!
//! let re_encoded = decoded.compress::<ULaw>();
//! assert_eq!(re_encoded, ENCODED);
//! ```
//!
//! [1]: https://en.wikipedia.org/wiki/%CE%9C-law_algorithm
//! [2]: https://en.wikipedia.org/wiki/A-law_algorithm
//! [3]: https://www.itu.int/rec/T-REC-G.711-198811-I/en
//! [4]: https://github.com/openitu/STL
//! [5]: https://github.com/openitu/STL/tree/dev/src/g711
//! [6]: `crate::traits::Compressed::expand`
//! [7]: `crate::traits::Expanded::compress`
//! [8]: `crate::traits::CompressedBuf::expand_buf`
//! [9]: `crate::traits::ExpandedBuf::compress_buf`
//! [10]: `crate::ALaw`
//! [11]: `crate::ULaw`
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "g191", feature = "g191-sys"))]
pub mod g191;
mod impls;
pub mod traits;

use crate::traits::Compander;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        let sign = 1 - i16::from(2 * u8::from(log < 0b1000_0000));
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

pub mod prelude {
    #[cfg(feature = "g191")]
    pub use crate::g191;
    pub use crate::traits::*;
    pub use crate::ALaw;
    pub use crate::ULaw;
}
