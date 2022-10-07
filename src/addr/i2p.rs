use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct I2pAddr([u8; 32]);

impl FromStr for I2pAddr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Display for I2pAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}