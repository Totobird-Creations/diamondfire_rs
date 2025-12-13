//! Extern function name mappings.


use std::{
    collections::hash_map::{
        HashMap,
        Entry as HashMapEntry
    },
    io::Write
};
use bincode::{ Encode, Decode };


#[derive(Encode, Decode)]
pub struct ExternNameMap {
    pub names : HashMap<String, ExternName>
}

#[derive(PartialEq, Eq, Encode, Decode, Debug)]
pub enum ExternName {
    OpaqueTy,
    ValueTy(ValueTy),
    NewVar(VarScope),
    ConstValue(ValueTy),
    Action {
        codeblock    : ActionBlockKind,
        action       : String,
        tag_defaults : Vec<String>
    },
    Gamevalue {
        gamevalue : String
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Debug)]
pub enum ActionBlockKind {
    PlayerAction,
    NpcAction,
    SetVar,
    GameAction,
    SelectEntity,
    Control
}
impl From<&str> for ActionBlockKind {
    fn from(value : &str) -> Self { match (value) {
        "PLAYER ACTION" => Self::PlayerAction,
        "ENTITY ACTION" => Self::NpcAction,
        "SET VARIABLE"  => Self::SetVar,
        "GAME ACTION"   => Self::GameAction,
        "SELECT OBJECT" => Self::SelectEntity,
        "CONTROL"       => Self::Control
        ,_              => unreachable!("{:?}", value)
    } }
}

#[derive(PartialEq, Eq, Encode, Decode, Debug)]
pub enum ValueTy {
    String,
    Text,
    Number,
    Location,
    Vector,
    Sound,
    Particle,
    Potion,
    Item,
    List,
    Dict
}

#[derive(PartialEq, Eq, Encode, Decode, Debug)]
pub enum VarScope {
    Local,
    ThreadLocal,
    Session,
    Persistent
}


impl Default for ExternNameMap {
    fn default() -> Self {
        let mut enm = Self { names : HashMap::new() };
        enm.declare("df_opaque",   ExternName::OpaqueTy);
        enm.declare("df_string",   ExternName::ValueTy(ValueTy::String));
        enm.declare("df_text",     ExternName::ValueTy(ValueTy::Text));
        enm.declare("df_number",   ExternName::ValueTy(ValueTy::Number));
        enm.declare("df_location", ExternName::ValueTy(ValueTy::Location));
        enm.declare("df_vector",   ExternName::ValueTy(ValueTy::Vector));
        enm.declare("df_sound",    ExternName::ValueTy(ValueTy::Sound));
        enm.declare("df_particle", ExternName::ValueTy(ValueTy::Particle));
        enm.declare("df_potion",   ExternName::ValueTy(ValueTy::Potion));
        enm.declare("df_item",     ExternName::ValueTy(ValueTy::Item));
        enm.declare("df_list",     ExternName::ValueTy(ValueTy::List));
        enm.declare("df_dict",     ExternName::ValueTy(ValueTy::Dict));
        enm.declare("DF_VAR__NewLocal",       ExternName::NewVar(VarScope::Local));
        enm.declare("DF_VAR__NewThreadLocal", ExternName::NewVar(VarScope::ThreadLocal));
        enm.declare("DF_VAR__NewSession",     ExternName::NewVar(VarScope::Session));
        enm.declare("DF_VAR__NewPersistent",  ExternName::NewVar(VarScope::Persistent));
        enm.declare("DF_CONST__String",   ExternName::ConstValue(ValueTy::String));
        enm.declare("DF_CONST__Text",     ExternName::ConstValue(ValueTy::Text));
        enm.declare("DF_CONST__Number",   ExternName::ConstValue(ValueTy::Number));
        enm.declare("DF_CONST__Location", ExternName::ConstValue(ValueTy::Location));
        enm.declare("DF_CONST__Vector",   ExternName::ConstValue(ValueTy::Vector));
        enm.declare("DF_CONST__Sound",    ExternName::ConstValue(ValueTy::Sound));
        enm.declare("DF_CONST__Particle", ExternName::ConstValue(ValueTy::Particle));
        enm.declare("DF_CONST__Potion",   ExternName::ConstValue(ValueTy::Potion));
        enm.declare("DF_CONST__Item",     ExternName::ConstValue(ValueTy::Item));
        enm
    }
}

impl ExternNameMap {

    pub fn declare<S : Into<String>>(&mut self, name : S, value : ExternName) -> bool {
        match (self.names.entry(name.into())) {
            HashMapEntry::Occupied(_) => {
                // assert_eq!(entry.get(), &value);
                false
            },
            HashMapEntry::Vacant(entry) => {
                entry.insert(value);
                true
            }
        }
    }

    pub fn encode_write<W : Write>(&self, f : &mut W) -> Result<(), bincode::error::EncodeError> {
        bincode::encode_into_std_write(self, f, bincode::config::standard()).map(|_| ())
    }

}
