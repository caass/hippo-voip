use crate::traits::{BufCompander, Compander, Compressed, CompressedBuf, Expanded, ExpandedBuf};

impl Compressed for u8 {
    type Expanded = i16;
}

impl Expanded for i16 {
    type Compressed = u8;
}

impl<const N: usize> Compressed for [u8; N] {
    type Expanded = [i16; N];
}

impl<const N: usize> Expanded for [i16; N] {
    type Compressed = [u8; N];
}

impl<const N: usize, T> Compander<[i16; N], [u8; N]> for T
where
    T: BufCompander<[i16], [u8]>,
{
    #[inline]
    fn compress(linear: [i16; N]) -> [u8; N] {
        let mut log = [0; N];
        linear.compress_buf::<Self>(&mut log);
        log
    }

    #[inline]
    fn expand(log: [u8; N]) -> [i16; N] {
        let mut linear = [0; N];
        log.expand_buf::<Self>(&mut linear);
        linear
    }
}

impl CompressedBuf for [u8] {
    type ExpandedBuf = [i16];
}

impl ExpandedBuf for [i16] {
    type CompressedBuf = [u8];
}

impl<T> BufCompander<[i16], [u8]> for T
where
    T: Compander<i16, u8>,
{
    #[inline]
    fn compress_buf(linear_buf: &[i16], log_buf: &mut [u8]) -> usize {
        linear_buf
            .iter()
            .copied()
            .zip(log_buf.iter_mut())
            .for_each(|(linear, slot)| *slot = linear.compress::<Self>());

        usize::min(linear_buf.len(), log_buf.len())
    }

    #[inline]
    fn expand_buf(log_buf: &[u8], linear_buf: &mut [i16]) -> usize {
        log_buf
            .iter()
            .copied()
            .zip(linear_buf.iter_mut())
            .for_each(|(log, slot)| *slot = log.expand::<Self>());

        usize::min(log_buf.len(), linear_buf.len())
    }
}
