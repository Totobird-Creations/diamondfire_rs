use crate::dfmir::DfMirStmt;
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

    /// The source name of the function, including crate name and module path.
    pub src_name : String,

    /// The source doc comments of the function.
    pub src_doc  : String,

    /// The name of the function.
    /// This will have already been mangled.
    pub name     : String,

    /// Prevents the function from being renamed, removed, inlined, or having its signature modified.
    pub exported : bool,

    /// Whether the function should be inlined.
    /// This is ignored if `exported` is `true`.
    pub inline   : FunctionItemInline,

    pub body     : Vec<DfMirStmt>

}


#[derive(Encode, Decode, Debug)]
pub enum FunctionItemInline {

    /// Determine whether the function should be inlined based on dfJSON cost, usages, etc.
    Maybe,

    /// Never inline the function.
    Never,

    /// Always inline the function.
    ///
    /// This will be ignored if a pointer to the function is acquired, or this function can call itself directly or indirectly.
    Always

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

    pub fn append(&mut self, other : &mut BridgeItems) {
        if let Some(extern_names) = other.extern_names.take() {
            assert!(self.extern_names.is_none());
            self.extern_names = Some(extern_names);
        }
        self.functions.append(&mut other.functions);
    }

}
