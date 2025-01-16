use core::ptr;

use crate::Compander;

#[cfg_attr(feature = "g191-sys", visibility::make(pub))]
mod sys {
    use core::ffi::{c_long, c_short};

    unsafe extern "C" {
        /// `ALaw` encoding rule according ITU-T Rec. G.711.
        ///
        /// Reads `lseg` 12-bit linear samples (left-aligned) from `linbuf`, and writes
        /// the 8-bit compressed samples (right-aligned) out to `logbuf`.
        ///
        /// # Safety
        ///
        /// The `linbuf` pointer must point to a continuous chunk of data that's valid for `lseg`
        /// reads of [`c_short`]s. Similarly, the `logbuf` pointer must point to a continuous chunk
        /// of data that's valid for `lseg` writes of [`c_short`]s.
        ///
        /// ```
        /// use core::ptr;
        /// use amuse::g191;
        ///
        /// let linear = [-0x5EE4, 0x48A7, 0x1430, -0x35B8, -0x4A54, -0x39EA, -0x74E0, 0x0036];
        /// let mut compressed = [0; 8];
        /// # assert_eq!(linear.len(), 8);
        /// # assert_eq!(compressed.len(), 8);
        ///
        /// let lseg = 8;
        /// let linbuf = linear.as_ptr();
        /// let logbuf = compressed.as_mut_ptr();
        ///
        /// unsafe { g191::sys::alaw_compress(lseg, linbuf, logbuf) };
        ///
        /// assert_eq!(compressed, [0x22, 0xA7, 0x81, 0x3F, 0x27, 0x39, 0x28, 0xD6]);
        /// ```
        ///
        /// The original documentation for this function is as follows:
        ///
        /// ```text
        ///    FUNCTION NAME: alaw_compress
        ///
        ///    DESCRIPTION: ALaw encoding rule according ITU-T Rec. G.711.
        ///
        ///    PROTOTYPE: void alaw_compress(long lseg, short *linbuf, short *logbuf)
        ///
        ///    PARAMETERS:
        ///      lseg:      (In)  number of samples
        ///      linbuf:	(In)  buffer with linear samples (only 12 MSBits are taken
        ///                       into account)
        ///      logbuf:	(Out) buffer with compressed samples (8 bit right justified,
        ///                       without sign extension)
        ///
        ///    RETURN VALUE: none.
        ///
        ///    HISTORY:
        ///    10.Dec.91	1.0	Separated A-law compression function
        /// ```
        pub fn alaw_compress(lseg: c_long, linbuf: *const c_short, logbuf: *mut c_short);

        /// `ALaw` decoding rule according ITU-T Rec. G.711.
        ///
        /// Reads `lseg` 8-bit compressed samples (right-aligned) from `logbuf`, and writes
        /// the 12-bit linear samples (left-aligned) out to `linbuf`.
        ///
        /// # Safety
        ///
        /// The `logbuf` pointer must point to a continuous chunk of data that's valid for `lseg`
        /// reads of [`c_short`]s. Similarly, the `linbuf` pointer must point to a continuous chunk
        /// of data that's valid for `lseg` writes of [`c_short`]s.
        ///
        /// ```
        /// use core::ptr;
        /// use amuse::g191;
        ///
        /// let compressed = [0x22, 0xA7, 0x81, 0x3F, 0x27, 0x39, 0x28, 0xD6];
        /// let mut linear = [0; 8];
        /// # assert_eq!(compressed.len(), 8);
        /// # assert_eq!(linear.len(), 8);
        ///
        /// let lseg = 8;
        /// let logbuf = compressed.as_ptr();
        /// let linbuf = linear.as_mut_ptr();
        ///
        /// unsafe { g191::sys::alaw_expand(lseg, logbuf, linbuf) };
        ///
        /// assert_eq!(linear, [-0x5E00, 0x4A00, 0x1480, -0x3500, -0x4A00, -0x3900, -0x7600, 0x0038]);
        /// ```
        ///
        /// The original documentation for this function is as follows:
        ///
        /// ```text
        /// FUNCTION NAME: alaw_expand
        ///
        /// DESCRIPTION: ALaw decoding rule according ITU-T Rec. G.711.
        ///
        /// PROTOTYPE: void alaw_expand(long lseg, short *logbuf, short *linbuf)
        ///
        /// PARAMETERS:
        ///   lseg:	    (In)  number of samples
        ///   logbuf:	(In)  buffer with compressed samples (8 bit right justified,
        ///                    without sign extension)
        ///   linbuf:	(Out) buffer with linear samples (13 bits left justified)
        ///
        /// RETURN VALUE: none.
        ///
        /// HISTORY:
        /// 10.Dec.91	1.0	Separated A-law expansion function
        /// ```
        pub fn alaw_expand(lseg: c_long, logbuf: *const c_short, linbuf: *mut c_short);

        pub fn ulaw_compress(lseg: c_long, linbuf: *const c_short, logbuf: *mut c_short);

        pub fn ulaw_expand(lseg: c_long, logbuf: *const c_short, linbuf: *mut c_short);
    }
}

#[cfg(feature = "g191")]
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ULaw;

#[cfg(feature = "g191")]
impl Compander for ULaw {
    fn compress(linear: i16) -> u8 {
        let mut log = 0i16;

        unsafe {
            sys::ulaw_compress(1, ptr::from_ref(&linear), ptr::from_mut(&mut log));
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
                ptr::from_ref(&i16::from(log)),
                ptr::from_mut(&mut linear),
            );
        }

        linear
    }
}

#[cfg(feature = "g191")]
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ALaw;

#[cfg(feature = "g191")]
impl Compander for ALaw {
    fn compress(linear: i16) -> u8 {
        let mut log = 0i16;

        unsafe {
            sys::alaw_compress(1, ptr::from_ref(&linear), ptr::from_mut(&mut log));
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
            sys::alaw_expand(
                1,
                ptr::from_ref(&i16::from(log)),
                ptr::from_mut(&mut linear),
            );
        }

        linear
    }
}
