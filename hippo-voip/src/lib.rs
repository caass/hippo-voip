use thiserror::Error;

use amuse::{Compressed, ULaw};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
}

pub struct Hippo<E> {
    buf: Vec<i16>,
    base64: E,
}

impl<E: base64::Engine> Hippo<E> {
    pub fn process_base64_ulaw<T: AsRef<[u8]>>(
        &mut self,
        encoded: T,
    ) -> Result<Option<String>, Error> {
        self.base64
            .decode(encoded)?
            .expand_into::<ULaw>(&mut self.buf);
        self.drive()
    }

    fn drive(&mut self) -> Result<Option<String>, Error> {
        todo!()
    }
}
