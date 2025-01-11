#![no_std]

pub fn compress(linear: i16) -> u8 {
    let is_negative = linear.is_negative();
    let absno = i16::min(
        (if is_negative { !linear } else { linear } >> 2) + 33,
        0x1FFF,
    );
    let segno = 11 - (absno.leading_zeros() as u8);

    ((0b1000 - segno) << 4)
        | (0b1111 - (((absno >> segno) as u8) & 0b1111))
        | (((!is_negative) as u8) << 7)
}

pub fn compress_slice(linear_buf: &[i16], log_buf: &mut [u8]) -> usize {
    linear_buf
        .iter()
        .copied()
        .zip(log_buf.iter_mut())
        .for_each(|(linear, slot)| *slot = compress(linear));

    usize::min(linear_buf.len(), log_buf.len())
}

pub fn expand(log: u8) -> i16 {
    let sign = if log < 0b1000_0000 { -1i16 } else { 1 };

    let mut mantissa = !log;
    let exponent = (mantissa >> 4) & 0b111;
    mantissa &= 0b1111;

    let step = 4i16 << (exponent + 1);

    sign * ((0b1000_0000 << exponent) + (step * i16::from(mantissa)) + (step / 2) - (4 * 33))
}

pub fn expand_slice(log_buf: &[u8], linear_buf: &mut [i16]) -> usize {
    log_buf
        .iter()
        .copied()
        .zip(linear_buf.iter_mut())
        .for_each(|(log, slot)| *slot = expand(log));

    usize::min(log_buf.len(), linear_buf.len())
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
mod test {
    extern crate alloc; // req'd for proptest
    use alloc::format;

    use crate::g711;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn compress(linear: i16) {
            let expected = crate::compress(linear);
            let actual = g711::compress(linear);

            prop_assert_eq!(expected, actual);
        }

        #[test]
        fn compress_slice(linear in prop::array::uniform32(i16::MIN..)) {
            let mut expected = [0u8; 32];
            let mut actual = [0u8; 32];

            g711::compress_slice(&linear, &mut [0i16; 32], &mut expected);
            crate::compress_slice(&linear, &mut actual);

            prop_assert_eq!(expected, actual);
        }

        #[test]
        fn expand(log: u8) {
            let expected = crate::expand(log);
            let actual = g711::expand(log);

            prop_assert_eq!(expected, actual);
        }

        #[test]
        fn expand_slice(log in prop::array::uniform32(u8::MIN..)) {
            let mut expected = [0; 32];
            let mut actual = [0; 32];

            g711::expand_slice(&log, &mut [0i16; 32], &mut expected);
            crate::expand_slice(&log, &mut actual);

            prop_assert_eq!(expected, actual);
        }
    }
}
