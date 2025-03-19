#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(any(feature = "g191", feature = "g191-sys"))]
pub mod g191;

pub trait Compander {
    #[must_use]
    fn compress(linear: i16) -> u8;

    #[must_use]
    fn expand(log: u8) -> i16;

    #[cfg(feature = "alloc")]
    #[inline]
    fn expand_into<T: AsRef<[u8]> + ?Sized>(log: &T, linear: &mut Vec<i16>) {
        let log = log.as_ref();

        linear.reserve(log.len());
        linear.extend(log.iter().copied().map(Self::expand));
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn compress_into<T: AsRef<[i16]> + ?Sized>(linear: &T, log: &mut Vec<u8>) {
        let linear = linear.as_ref();

        log.reserve(linear.len());
        log.extend(linear.iter().copied().map(Self::compress));
    }
}

pub trait Compressed: AsRef<[u8]> {
    #[cfg(feature = "alloc")]
    #[inline]
    fn expand<C: Compander>(&self) -> Vec<i16> {
        let mut linear = Vec::new();
        self.expand_into::<C>(&mut linear);
        linear
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn expand_into<C: Compander>(&self, linear: &mut Vec<i16>) {
        C::expand_into(self, linear);
    }
}

impl<T: AsRef<[u8]>> Compressed for T {}

pub trait Expanded: AsRef<[i16]> {
    #[cfg(feature = "alloc")]
    #[inline]
    fn compress<C: Compander>(&self) -> Vec<u8> {
        let mut log = Vec::new();
        self.compress_into::<C>(&mut log);
        log
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn compress_into<C: Compander>(&self, log: &mut Vec<u8>) {
        C::compress_into(self, log);
    }
}

impl<T: AsRef<[i16]>> Expanded for T {}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ULaw;

impl Compander for ULaw {
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
        let high_nibble = (0b1000 - segment) << 4;
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

impl Compander for ALaw {
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
        let sign = if log < 0b1000_0000 { -1 } else { 1 };
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
