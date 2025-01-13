use std::io::{Read, Result, Write};

use crate::traits::{BufCompander, Compander, Compressed, CompressedBuf, ExpandedBuf};

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
        let mut log = vec![];
        let n = self.read_to_end(&mut log)?;

        *linear = log.expand::<C>();
        Ok(n)
    }
}
