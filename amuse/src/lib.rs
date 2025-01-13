#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod alaw;
#[cfg(feature = "g191")]
pub mod g191;
mod impls;
pub mod traits;
mod ulaw;

pub use alaw::ALaw;
pub use ulaw::ULaw;

pub mod prelude {
    #[cfg(feature = "std")]
    pub use crate::impls::std::{ReadExt, WriteExt};
    pub use crate::traits::*;
}
