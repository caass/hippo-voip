trait Encoder {
    fn compress(&mut self, linear: &[i16], log: &mut [u8]) -> usize;

    fn expand(&mut self, linear: &[i16], log: &mut [u8]) -> usize;
}

mod reference {

    mod ffi {
        use core::ffi::{c_long, c_short};

        unsafe extern "C" {
            pub fn alaw_compress(lseg: c_long, linbuf: *mut c_short, logbuf: *mut c_short);

            pub fn alaw_expand(lseg: c_long, logbuf: *mut c_short, linbuf: *mut c_short);

            pub fn ulaw_compress(lseg: c_long, linbuf: *mut c_short, logbuf: *mut c_short);

            pub fn ulaw_expand(lseg: c_long, logbuf: *mut c_short, linbuf: *mut c_short);
        }
    }
}
