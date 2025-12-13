//! Extern function name mappings.


use std::{
    collections::HashMap,
    io::Write
};
use bincode::{ Encode, Decode };


#[derive(Encode, Decode, Default)]
pub struct ExternNameMap {
    pub names : HashMap<String, ExternName>
}

#[derive(Encode, Decode)]
pub enum ExternName {
    Action {
        codeblock     : CodeBlockType,
        action        : String,
        tags_defaults : Vec<String>
    }
}

#[derive(Encode, Decode)]
pub enum CodeBlockType {
}


impl ExternNameMap {
    pub fn encode_write<W : Write>(&self, f : &mut W) -> Result<(), bincode::error::EncodeError> {
        bincode::encode_into_std_write(self, f, bincode::config::standard()).map(|_| ())
    }
}
