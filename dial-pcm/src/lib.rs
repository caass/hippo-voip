#![cfg_attr(not(feature = "std"), no_std)]

pub fn compress(linear: i16) -> u8 {
    const I14_MAX: i16 = 0b01_1111_1111_1111;
    const I14_SHIFT_OFFSET: i16 = 2;
    const MULAW_OFFSET: i16 = 33;

    const NUM_SEGMENTS: u8 = 11;

    const HIGH_NIBBLE_OFFSET: u8 = 4;
    const HIGH_NIBBLE_MAX: u8 = 0b1000;

    const LOW_NIBBLE_MAX: u8 = 0b1111;
    const LOW_NIBBLE_MASK: u8 = 0b1111;

    const SIGN_BIT_OFFSET: u8 = 7;

    let is_negative = linear.is_negative();
    let absno = i16::min(
        (if is_negative { !linear } else { linear } >> I14_SHIFT_OFFSET) + MULAW_OFFSET,
        I14_MAX,
    );
    let segno = NUM_SEGMENTS - (absno.leading_zeros() as u8);

    ((HIGH_NIBBLE_MAX - segno) << HIGH_NIBBLE_OFFSET)
        | (LOW_NIBBLE_MAX - (((absno >> segno) as u8) & LOW_NIBBLE_MASK))
        | (((!is_negative) as u8) << SIGN_BIT_OFFSET)
}

pub fn expand(log: u8) -> i16 {
    let sign = if log < 0b1000_0000 { -1i16 } else { 1 };

    let mut mantissa = !log;
    let exponent = (mantissa >> 4) & 0b111;
    mantissa &= 0b1111;

    let step = 4i16 << (exponent + 1);

    sign * ((0b1000_0000 << exponent) + (step * i16::from(mantissa)) + (step / 2) - (4 * 33))
}

pub mod g711 {
    use core::slice;

    mod sys {
        include!(env!("G711_H_RS"));
    }

    pub fn compress(linear: i16) -> u8 {
        let mut buf = [0];
        compress_slice(slice::from_ref(&linear), &mut [0], &mut buf);
        buf[0]
    }

    pub fn compress_slice(linear: &[i16], scratch: &mut [i16], log: &mut [u8]) -> usize {
        let linbuf = linear.as_ptr().cast_mut();
        let logbuf = scratch.as_mut_ptr();
        let k = linear.len().min(scratch.len()).min(log.len());
        let lseg = k.try_into().unwrap_or(i64::MAX);

        unsafe { sys::ulaw_compress(lseg, linbuf, logbuf) };

        for (val, slot) in scratch.iter().copied().zip(log.iter_mut()).take(k) {
            *slot = val as u8;
        }

        k
    }

    pub fn expand(log: u8) -> i16 {
        let mut buf = [0];
        expand_slice(slice::from_ref(&log), &mut [0], &mut buf);
        buf[0]
    }

    pub fn expand_slice(log: &[u8], scratch: &mut [i16], linear: &mut [i16]) -> usize {
        for (val, slot) in log.iter().copied().zip(scratch.iter_mut()) {
            *slot = val.into();
        }

        let logbuf = scratch.as_mut_ptr();
        let linbuf = linear.as_ptr().cast_mut();
        let k = linear.len().min(scratch.len()).min(log.len());
        let lseg = k.try_into().unwrap_or(i64::MAX);

        unsafe { sys::ulaw_expand(lseg, logbuf, linbuf) };

        k
    }
}

#[cfg(test)]
mod compare_impls {
    use crate::g711;

    #[test]
    fn compress() {
        for linear in -8159..8159 {
            let expected = g711::compress(linear);
            let actual = crate::compress(linear);

            assert_eq!(expected, actual, "Mismatch between reference implementation and dial implementation when compressing {linear}");
        }
    }

    #[test]
    fn expand() {
        for log in u8::MIN..=u8::MAX {
            let expected = g711::expand(log);
            let actual = crate::expand(log);

            assert_eq!(expected, actual, "Mismatch between reference implementation and dial implementation when expanding {log}");
        }
    }
}
