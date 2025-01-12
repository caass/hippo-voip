use crate::traits::Compander;

#[derive(Debug, Clone, Copy, Default)]
pub struct ALaw;

impl Compander<i16, u8> for ALaw {
    fn compress(linear: i16) -> u8 {
        todo!()
    }

    fn expand(log: u8) -> i16 {
        todo!()
    }
}
