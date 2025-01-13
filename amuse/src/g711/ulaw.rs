use core::ptr;

use crate::g711::sys;
use crate::traits::Compander;

#[derive(Debug, Clone, Copy, Default)]
pub struct ULaw;

impl Compander<i16, u8> for ULaw {
    fn compress(linear: i16) -> u8 {
        let mut log = 0i16;

        unsafe {
            sys::ulaw_compress(
                1,
                ptr::from_ref(&linear).cast_mut(),
                ptr::from_mut(&mut log),
            );
        }

        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation,
            reason = "the compressed value only uses the lower 8 bits"
        )]
        let out = log as u8;

        out
    }

    fn expand(log: u8) -> i16 {
        let mut linear = 0i16;

        unsafe {
            sys::ulaw_expand(
                1,
                ptr::from_ref(&i16::from(log)).cast_mut(),
                ptr::from_mut(&mut linear),
            );
        }

        linear
    }
}
