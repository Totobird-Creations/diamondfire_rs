//! DiamondFire Mid-Level Intermediate Representation
//!
//! dfMIR is quite similar to rsMIR but with different statement types.

use core::{
    fmt::{ self, Debug, Formatter },
    panic::Location
};
use bincode::{ Encode, Decode };

mod ty;
pub use ty::*;


#[derive(Encode, Decode)]
pub enum DfMirStmt {

    CopyTL {
        dst : DfMirLocal,
        src : DfMirTemporary
    },
    CopyLT {
        dst : DfMirTemporary,
        src : DfMirLocal
    },
    CopyTT {
        dst : DfMirTemporary,
        src : DfMirTemporary
    },

    TNumber {
        dst      : DfMirTemporary,
        /// Representation depends on type.
        df_value : i64
    },
    TStruct {
        dst    : DfMirTemporary,
        fields : Vec<DfMirTemporary>
    },
    TEnum {
        dst     : DfMirTemporary,
        variant : DfMirTemporary,
        fields  : Vec<DfMirTemporary>
    },

    RawFieldGet {
        dst   : DfMirTemporary,
        src   : DfMirTemporary,
        field : usize
    },

    Call {
        dst  : DfMirTemporary,
        call : DfMirCall,
        args : Vec<DfMirTemporary>
        // TODO: Returns
    },
    DropCall {
        fn_id : u128,
        /// This value needs to be `&mut` before calling.
        value : DfMirTemporary
    },

    Return,

    Todo(String) // TODO: Remove

}
impl DfMirStmt {
    #[track_caller]
    pub fn todo(msg : &str) -> Self {
        let loc = Location::caller();
        Self::Todo(format!("{}:{}:{} {}", loc.file(), loc.line(), loc.column(), msg))
    }
}
impl Debug for DfMirStmt { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    match (self) {

        Self::CopyTL { dst, src } => { write!(f, "{:?} = {:?}", dst, src)?; },
        Self::CopyLT { dst, src } => { write!(f, "{:?} = {:?}", dst, src)?; },
        Self::CopyTT { dst, src } => { write!(f, "{:?} = {:?}", dst, src)?; },

        Self::TNumber { dst, df_value } => { write!(f, "{:?} = {:?}", dst, df_value)?; },
        Self::TStruct { dst, fields }   => {
            write!(f, "{:?} = struct {{ ", dst)?;
            for field in fields {
                write!(f, "{:?}, ", field)?;
            }
            write!(f, "}}")?;
        },
        Self::TEnum { dst, variant, fields } => {
            write!(f, "{:?} = enum::{:?} {{ ", dst, variant)?;
            for field in fields {
                write!(f, "{:?}, ", field)?;
            }
            write!(f, "}}")?;
        },

        Self::RawFieldGet { dst, src, field } => {
            write!(f, "{:?} = {:?}.{}", dst, src, field)?;
        },

        Self::Call { dst, call, args } => {
            write!(f, "{:?} = {:?}( ", dst, call)?;
            for arg in args {
                write!(f, "{:?}, ", arg)?;
            }
            write!(f, ")")?;
        },
        Self::DropCall { fn_id, value } => {
            write!(f, "drop.{}( {:?}, )", fn_id, value)?;
        }

        Self::Return => { write!(f, "return")?; },

        Self::Todo(msg) => { write!(f, "TODO: {}", msg)?; }

    }
    Ok(())
} }

#[derive(Encode, Decode)]
pub enum DfMirCall {
    Defined(u128),
    Ptr(DfMirTemporary),
    Extern(String),
    Intrinsic(DfMirCallIntrinsic)
}
impl Debug for DfMirCall { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    match (self) {
        Self::Defined(func_id) => { write!(f, "{}", func_id)?; },
        Self::Ptr(ptr)         => { write!(f, "*{:?}", ptr)?; },
        Self::Extern(name)     => { write!(f, "extern.{:?}", name)?; },
        Self::Intrinsic(name)  => { write!(f, "intrinsic.{:?}", name)?; }
    }
    Ok(())
} }

#[derive(Clone, Copy, Encode, Decode)]
pub struct DfMirLocal(pub usize);
impl Debug for DfMirLocal { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    write!(f, "local_{}", self.0)
} }

#[derive(Clone, Copy, Encode, Decode)]
pub struct DfMirTemporary(pub usize);
impl DfMirTemporary {
    pub const PLACEHOLDER : Self = Self(usize::MAX); // TODO: Remove
}
impl Debug for DfMirTemporary { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    write!(f, "temp_{}", self.0)
} }

#[derive(Encode, Decode, Debug)]
pub enum DfMirCallIntrinsic {
    Abort,
    AbsF32,
    AbsF64,
    ByteReverse,
    CallerLocation,
    CeilF32,
    CeilF64,
    CompareBytes,
    CopySignF64,
    CountLeadingZeroBits,
    CountOneBits,
    CountTrailingZeroBits,
    FloorF32,
    FloorF64,
    FunnelShlUnchecked,
    DisjointBitOr,
    MulAddF64,
    PtrShift,
    PtrOffsetFromUnsigned,
    RoundTiesEvenF32,
    RoundTiesEvenF64,
    SaturatingSub,
    SelectUnpredictable,
    SqrtF32,
    SqrtF64,
    TruncF32,
    TruncF64
}


impl DfMirStmt {

    // /// Approximates the total number of dfJSON code blocks that several statements will become.
    // ///
    // /// Used to determine if a function is applicable for inlining.
    // pub fn dfjson_cost_many(stmts : impl IntoIterator<Item = DfMirStmt>) -> usize {
    //     stmts.into_iter().map(|stmt| stmt.dfjson_cost()).sum()
    // }

    // /// Approximates the number of dfJSON code blocks this statement will become.
    // pub fn dfjson_cost(&self) -> usize {
    //     match (self) {

    //         Self::CopyTL { .. } => 1,
    //         Self::CopyLT { .. } => 1,
    //         Self::CopyTT { .. } => 1,

    //         Self::TNumber { .. }         => 1,
    //         Self::TStruct { fields, .. } => fields.len().div_ceil(26) + 1,
    //         Self::TEnum { fields, .. }   => (fields.len() + 1).div_ceil(26) + 1,

    //         Self::RawFieldGet { .. } => 1,

    //         Self::Call { call, .. }      => { match (call) {
    //             DfMirCall::Defined(_) => 1,
    //             DfMirCall::Ptr(_)     => 1,
    //             DfMirCall::Extern(_)  => 1,
    //             DfMirCall::Intrinsic(intrinsic) => { match (intrinsic) {
    //                 DfMirCallIntrinsic::Abort                 => todo!(),
    //                 DfMirCallIntrinsic::AbsF32                => todo!(),
    //                 DfMirCallIntrinsic::AbsF64                => todo!(),
    //                 DfMirCallIntrinsic::ByteReverse           => todo!(),
    //                 DfMirCallIntrinsic::CallerLocation        => todo!(),
    //                 DfMirCallIntrinsic::CeilF32               => todo!(),
    //                 DfMirCallIntrinsic::CeilF64               => todo!(),
    //                 DfMirCallIntrinsic::CompareBytes          => todo!(),
    //                 DfMirCallIntrinsic::CopySignF64           => todo!(),
    //                 DfMirCallIntrinsic::CountLeadingZeroBits  => todo!(),
    //                 DfMirCallIntrinsic::CountOneBits          => todo!(),
    //                 DfMirCallIntrinsic::CountTrailingZeroBits => todo!(),
    //                 DfMirCallIntrinsic::FloorF32              => todo!(),
    //                 DfMirCallIntrinsic::FloorF64              => todo!(),
    //                 DfMirCallIntrinsic::FunnelShlUnchecked    => todo!(),
    //                 DfMirCallIntrinsic::DisjointBitOr         => todo!(),
    //                 DfMirCallIntrinsic::MulAddF64             => todo!(),
    //                 DfMirCallIntrinsic::PtrShift              => todo!(),
    //                 DfMirCallIntrinsic::PtrOffsetFromUnsigned => todo!(),
    //                 DfMirCallIntrinsic::RoundTiesEvenF32      => todo!(),
    //                 DfMirCallIntrinsic::RoundTiesEvenF64      => todo!(),
    //                 DfMirCallIntrinsic::SaturatingSub         => todo!(),
    //                 DfMirCallIntrinsic::SelectUnpredictable   => todo!(),
    //                 DfMirCallIntrinsic::SqrtF32               => todo!(),
    //                 DfMirCallIntrinsic::SqrtF64               => todo!(),
    //                 DfMirCallIntrinsic::TruncF32              => todo!(),
    //                 DfMirCallIntrinsic::TruncF64              => todo!()
    //             } }
    //         } },
    //         Self::DropCall { .. } => 1,

    //         Self::Return => 1
    //     }
    // }

}
