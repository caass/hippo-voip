#[cfg(feature = "std")]
use std::io::{Read, Result, Write};

pub trait Expanded {
    type Compressed: Compressed<Expanded = Self>;

    #[must_use]
    #[inline]
    fn compress<C: Compander<Self, Self::Compressed>>(self) -> Self::Compressed
    where
        Self: Sized,
    {
        C::compress(self)
    }
}

pub trait Compressed {
    type Expanded: Expanded<Compressed = Self>;

    #[must_use]
    #[inline]
    fn expand<C: Compander<Self::Expanded, Self>>(self) -> Self::Expanded
    where
        Self: Sized,
    {
        C::expand(self)
    }
}

pub trait Compander<E, C>
where
    E: Expanded<Compressed = C>,
    C: Compressed<Expanded = E>,
{
    #[must_use]
    fn compress(linear: E) -> C;

    #[must_use]
    fn expand(log: C) -> E;
}

pub trait ExpandedBuf {
    type CompressedBuf: CompressedBuf<ExpandedBuf = Self> + ?Sized;

    #[inline]
    fn compress_buf<C: BufCompander<Self, Self::CompressedBuf>>(
        &self,
        log: &mut Self::CompressedBuf,
    ) -> usize {
        C::compress_buf(self, log)
    }
}

pub trait CompressedBuf {
    type ExpandedBuf: ExpandedBuf<CompressedBuf = Self> + ?Sized;

    #[inline]
    fn expand_buf<C: BufCompander<Self::ExpandedBuf, Self>>(
        &self,
        linear: &mut Self::ExpandedBuf,
    ) -> usize {
        C::expand_buf(self, linear)
    }
}

pub trait BufCompander<E, C>
where
    E: ExpandedBuf<CompressedBuf = C> + ?Sized,
    C: CompressedBuf<ExpandedBuf = E> + ?Sized,
{
    fn compress_buf(linear_buf: &E, log_buf: &mut C) -> usize;

    fn expand_buf(log_buf: &C, linear_buf: &mut E) -> usize;
}

#[cfg(feature = "std")]
pub trait WriteExt: Write {
    fn write_compressed<C: BufCompander<[i16], [u8]>>(&mut self, linear: &[i16]) -> Result<usize> {
        let mut log = vec![0u8; linear.len()];
        linear.compress_buf::<C>(&mut log);
        self.write(&log)
    }

    fn write_all_compressed<C: BufCompander<[i16], [u8]>>(&mut self, linear: &[i16]) -> Result<()> {
        let mut log = vec![0u8; linear.len()];
        linear.compress_buf::<C>(&mut log);
        self.write_all(&log)
    }
}

#[cfg(feature = "std")]
pub trait ReadExt: Read {
    fn read_compressed<C: BufCompander<[i16], [u8]>>(
        &mut self,
        linear: &mut [i16],
    ) -> Result<usize> {
        let mut log = vec![0u8; linear.len()];
        let n = self.read(&mut log)?;
        log[..n].expand_buf::<C>(linear);

        Ok(n)
    }

    fn read_exact_compressed<C: BufCompander<[i16], [u8]>>(
        &mut self,
        linear: &mut [i16],
    ) -> Result<()> {
        let mut log = vec![0u8; linear.len()];
        self.read_exact(&mut log)?;
        log.expand_buf::<C>(linear);

        Ok(())
    }

    fn read_to_end_compressed<C: Compander<Vec<i16>, Vec<u8>>>(
        &mut self,
        linear: &mut Vec<i16>,
    ) -> Result<usize> {
        let mut log = Vec::with_capacity(linear.capacity());
        let n = self.read_to_end(&mut log)?;

        *linear = log.expand::<C>();
        Ok(n)
    }
}
