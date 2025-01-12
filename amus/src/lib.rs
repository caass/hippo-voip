#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod alaw;
#[cfg(feature = "g711")]
pub mod g711;
mod impls;
mod traits;
mod ulaw;

pub use alaw::ALaw;
pub use ulaw::ULaw;

pub mod prelude {
    pub use crate::alaw::ALaw;
    pub use crate::traits::*;
    pub use crate::ulaw::ULaw;
}

#[cfg(all(test, feature = "g711", feature = "alloc"))]
mod conformance {
    use alloc::format;

    use crate::g711;
    use crate::prelude::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn ulaw_compress(linear: i16) {
            let expected = linear.compress::<g711::ULaw>();
            let actual = linear.compress::<crate::ULaw>();

            prop_assert_eq!(expected, actual);
        }

        #[test]
        fn ulaw_expand(log: u8) {
            let expected = log.expand::<g711::ULaw>();
            let actual = log.expand::<crate::ULaw>();

            prop_assert_eq!(expected, actual);
        }

    }
}
