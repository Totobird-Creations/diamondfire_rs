use core::fmt::{ self, Debug, Formatter };
use std::{
    io::{ Read, Write },
    collections::BTreeMap
};
use bincode::{ Encode, Decode };


#[derive(Default, Encode, Decode)]
pub struct BridgeItems {
    pub extern_names : Option<Vec<u8>>,
    pub functions    : BTreeMap<u128, FunctionItem>
}


#[derive(Encode, Decode, Debug)]
pub struct FunctionItem {

    /// The name of the function.
    /// This will already have been mangled.
    pub name        : String,

    /// Whether this function can be inlined.
    pub can_inline  : bool,

    /// Whether the signature can be changed for optimisation.
    pub sig_mutable : bool

}


impl Debug for BridgeItems {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BridgeItems")
            .field_with("extern_names", |f| { if (self.extern_names.is_some()) { write!(f, "Some([...])") } else { write!(f, "None") } })
            .field("functions", &self.functions)
            .finish()
    }
}

impl BridgeItems {

    pub fn read_decode<R : Read>(f : &mut R) -> Result<Self, bincode::error::DecodeError> {
        bincode::decode_from_std_read(f, bincode::config::standard())
    }

    pub fn encode_write<W : Write>(&self, f : &mut W) -> Result<(), bincode::error::EncodeError> {
        bincode::encode_into_std_write(self, f, bincode::config::standard()).map(|_| ())
    }

}
