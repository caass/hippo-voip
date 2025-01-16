#![cfg(all(test, feature = "g191"))]

extern crate alloc;

use alloc::format;

use amuse::g191;
use amuse::prelude::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn ulaw(linear: [i16; 32]) {
        prop_assert_eq!(
            linear.compress::<g191::ULaw>(),
            linear.compress::<amuse::ULaw>(),
            "Mismatch in μ-law compression."
        );

        let log = linear.compress::<amuse::ULaw>();

        prop_assert_eq!(
            log.expand::<g191::ULaw>(),
            log.expand::<amuse::ULaw>(),
            "Mismatch in μ-law expansion."
        );
    }

    #[test]
    fn alaw(linear: [i16; 32]) {
        prop_assert_eq!(
            linear.compress::<g191::ALaw>(),
            linear.compress::<amuse::ALaw>(),
            "Mismatch in A-law compression."
        );

        let log = linear.compress::<amuse::ALaw>();

        prop_assert_eq!(
            log.expand::<g191::ALaw>(),
            log.expand::<amuse::ALaw>(),
            "Mismatch in A-law expansion."
        );
    }
}
