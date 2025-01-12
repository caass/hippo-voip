use alloc::vec;
use alloc::vec::Vec;

use crate::traits::{BufCompander, Compander, Compressed, CompressedBuf, Expanded, ExpandedBuf};

impl<E: Expanded> Expanded for Vec<E> {
    type Compressed = Vec<E::Compressed>;
}

impl<C: Compressed> Compressed for Vec<C> {
    type Expanded = Vec<C::Expanded>;
}

impl<T> Compander<Vec<i16>, Vec<u8>> for T
where
    T: BufCompander<[i16], [u8]>,
{
    #[inline]
    fn compress(linear: Vec<i16>) -> Vec<u8> {
        let mut log = vec![0; linear.len()];
        linear.compress_buf::<Self>(&mut log);
        log
    }

    #[inline]
    fn expand(log: Vec<u8>) -> Vec<i16> {
        let mut linear = vec![0; log.len()];
        log.expand_buf::<Self>(&mut linear);
        linear
    }
}
