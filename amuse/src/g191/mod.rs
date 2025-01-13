mod alaw;
mod ulaw;

pub use alaw::ALaw;
pub use ulaw::ULaw;

mod sys {
    use core::ffi::{c_long, c_short};

    unsafe extern "C" {
        pub(super) fn alaw_compress(lseg: c_long, linbuf: *const c_short, logbuf: *mut c_short);

        pub(super) fn alaw_expand(lseg: c_long, logbuf: *const c_short, linbuf: *mut c_short);

        pub(super) fn ulaw_compress(lseg: c_long, linbuf: *const c_short, logbuf: *mut c_short);

        pub(super) fn ulaw_expand(lseg: c_long, logbuf: *const c_short, linbuf: *mut c_short);
    }
}
