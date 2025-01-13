#![cfg(all(test, feature = "g191", feature = "alloc"))]

extern crate alloc;

use alloc::format;

use amuse::g191;
use amuse::prelude::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn ulaw_compress(linear: i16) {
        let expected = linear.compress::<g191::ULaw>();
        let actual = linear.compress::<amuse::ULaw>();

        prop_assert_eq!(expected, actual);
    }

    #[test]
    fn ulaw_expand(log: u8) {
        let expected = log.expand::<g191::ULaw>();
        let actual = log.expand::<amuse::ULaw>();

        prop_assert_eq!(expected, actual);
    }

    #[test]
    fn alaw_compress(linear: i16) {
        let expected = linear.compress::<g191::ALaw>();
        let actual = linear.compress::<amuse::ALaw>();

        prop_assert_eq!(expected, actual);
    }

    #[test]
    fn alaw_expand(log: u8) {
        let expected = log.expand::<g191::ALaw>();
        let actual = log.expand::<amuse::ALaw>();

        prop_assert_eq!(expected, actual);
    }

}
