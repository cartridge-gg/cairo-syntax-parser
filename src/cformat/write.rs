use starknet_types_core::felt::Felt;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for [u8; 32] {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("0x")?;
        for byte in self.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Display for [u8; 31] {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("0x00")?;
        for byte in self.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
