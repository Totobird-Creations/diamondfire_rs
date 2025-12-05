use super::DfMirFn;
use core::{
    fmt::{ self, Debug, Formatter },
    mem
};
use std::borrow::Cow;


pub struct DfMirPlace {
    pub local   : usize,
    pub project : Vec<DfMirPlaceElem>,
    pub ty      : Option<DfMirTy>
}

impl Debug for DfMirPlace {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "local.{}", self.local)?;
        for _elem in &self.project {
            todo!();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DfMirPlaceElem {}


#[derive(Debug)]
pub struct DfMirSlot {
    pub ty    : DfMirTy,
    pub refed : DfMirSlotRefState
}

#[derive(Debug)]
pub enum DfMirSlotRefState {

    /// The value is never referenced.
    /// It is only ever used directly as a local.
    None,

    /// The value has been referenced.
    Referenced,

    /// The value has been referenced.
    /// One or more references has been passed outside of the current scope.
    ///
    /// This forces the local to be stored in a `GAME` variable instead of `LINE`.
    Escaped

}


#[derive(Clone, Eq)]
pub enum DfMirTy {
    // Unknown,
    Str(DfMirStrTy),
    Text,
    Num(DfMirNumTy),
    Loc,
    Vec3,
    Sound,
    Par,
    Pot,
    List(Box<DfMirTy>),
    Dict(Box<DfMirTy>),
    Error
}

impl Debug for DfMirTy {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        match (self) {
            Self::Str(ty) => { match (ty) {
                DfMirStrTy::String => { write!(f, "Str") },
                DfMirStrTy::Char   => { write!(f, "Char") },
            } },
            Self::Text => { write!(f, "Text") },
            Self::Num(ty) => { match (ty) {
                DfMirNumTy::Float { size } => { write!(f, "F{}", size) },
                DfMirNumTy::Int { size, unsigned } => {
                    if (*unsigned) { write!(f, "I")?; } else { write!(f, "U")?; }
                    if let Some(size) = size { write!(f, "{}", size) } else { write!(f, "size") }
                },
                DfMirNumTy::Bool => { write!(f, "Bool") }
            } },
            Self::Loc      => { write!(f, "Loc") },
            Self::Vec3     => { write!(f, "Vec3") },
            Self::Sound    => { write!(f, "Sound") },
            Self::Par      => { write!(f, "Par") },
            Self::Pot      => { write!(f, "Pot") },
            Self::List(ty) => { write!(f, "List<{:?}>", ty) },
            Self::Dict(ty) => { write!(f, "Dict<{:?}>", ty) },
            Self::Error    => { write!(f, "error") }
        }
    }
}

impl PartialEq for DfMirTy {
    fn eq(&self, other : &Self) -> bool {
        match ((self, other,)) {
            // (Self::Unknown, _,) => false,
            // (_, Self::Unknown,) => false,
            (Self::Str(l),  Self::Str(r),)  => (l == r),
            (Self::Num(l),  Self::Num(r),)  => (l == r),
            (Self::List(l), Self::List(r),) => (l == r),
            (Self::Dict(l), Self::Dict(r),) => (l == r),
            _ => (mem::discriminant(self) == mem::discriminant(other))
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DfMirStrTy {
    String,
    Char
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DfMirNumTy {
    Float {
        size : u8
    },
    Int {
        /// If `None`, `isize` or `usize`.
        size     : Option<u64>,
        unsigned : bool
    },
    Bool
}


pub enum DfMirBasicVal {
    Temporary(usize),
    Local(usize),
    Int {
        value : i128,
        size  : u64
    },
    UInt {
        value : u128,
        size  : u64
    }
}

impl Debug for DfMirBasicVal {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        match (self) {
            Self::Temporary(temporary) => { write!(f, "temp.{}", temporary) },
            Self::Local(local)         => { write!(f, "local.{}", local) },
            Self::Int { value, size }  => { write!(f, "{}i{}", value, size) },
            Self::UInt { value, size } => { write!(f, "{}u{}", value, size) }
        }
    }
}

impl DfMirBasicVal {
    pub fn ty<'dmf>(&self, dest : &'dmf DfMirFn) -> Cow<'dmf, DfMirTy> { match (self) {
        Self::Temporary(temporary) => Cow::Borrowed(dest.get_temporary_ty(*temporary)),
        Self::Local(local)         => Cow::Borrowed(dest.get_local_ty(*local)),
        Self::Int { size, .. }     => Cow::Owned(DfMirTy::Num(DfMirNumTy::Int { size : Some(*size), unsigned : false })),
        Self::UInt { size, .. }    => Cow::Owned(DfMirTy::Num(DfMirNumTy::Int { size : Some(*size), unsigned : true }))
    } }
}
