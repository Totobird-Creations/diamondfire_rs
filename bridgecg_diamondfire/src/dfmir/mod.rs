//! DiamondFire Mid-Level Intermediate Representation
//!
//! dfMIR is quite similar to rsMIR but with different statement types.

use crate::{
    Local,
    Temporary
};
use core::{
    fmt::{ self, Debug, Formatter },
    panic::Location
};

mod ty;
pub use ty::*;


#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub enum DfMirStmt {

    CopyTL {
        dst : Local,
        src : Temporary
    },
    CopyLT {
        dst : Temporary,
        src : Local
    },
    CopyTT {
        dst : Temporary,
        src : Temporary
    },

    TNumber {
        dst        : Temporary,
        /// Representation depends on type.
        repr_value : i64
    },
    TString {
        dst   : Temporary,
        value : String
    },
    TStruct {
        dst    : Temporary,
        fields : Vec<Temporary>
    },
    TEnum {
        dst     : Temporary,
        variant : Temporary,
        fields  : Vec<Temporary>
    },

    CheckedArithBinOp {
        dst   : Temporary,
        op    : DfMirCheckedArithBinOp,
        left  : Temporary,
        right : Temporary
    },
    BoolBinOp {
        dst   : Temporary,
        op    : DfMirBoolBinOp,
        left  : Temporary,
        right : Temporary
    },
    CondBinOp {
        dst   : Temporary,
        op    : DfMirCondBinOp,
        left  : Temporary,
        right : Temporary
    },

    RawFieldGet {
        dst   : Temporary,
        src   : Temporary,
        field : usize
    },

    Call {
        dst  : Temporary,
        call : DfMirCall,
        args : Vec<Temporary>
        // TODO: Returns
    },
    DropCall {
        fn_id : u128,
        /// This value needs to be `&mut` before calling.
        value : Temporary
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

        Self::TNumber { dst, repr_value } => { write!(f, "{:?} = {:?}", dst, repr_value)?; },
        Self::TString { dst, value }      => { write!(f, "{:?} = {:?}", dst, value)?; },
        Self::TStruct { dst, fields }     => {
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

        Self::CheckedArithBinOp { dst, op, left, right } => {
            write!(f, "{:?} = {:?} {:?}? {:?}", dst, left, { match (op) {
                DfMirCheckedArithBinOp::Add => "+",
                DfMirCheckedArithBinOp::Shl => "<<"
            } }, right)?;
        },
        Self::BoolBinOp { dst, op, left, right } => {
            write!(f, "{:?} = {:?} {:?} {:?}", dst, left, { match (op) {
                DfMirBoolBinOp::Or  => "|",
                DfMirBoolBinOp::Xor => "^"
            } }, right)?;
        },
        Self::CondBinOp { dst, op, left, right } => {
            write!(f, "{:?} = {:?} {:?} {:?}", dst, left, { match (op) {
                DfMirCondBinOp::LessThan => "<"
            } }, right)?;
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

#[derive(Debug)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub enum DfMirCheckedArithBinOp {
    Add,
    Shl
}

#[derive(Debug)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub enum DfMirBoolBinOp {
    Or,
    Xor
}

#[derive(Debug)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub enum DfMirCondBinOp {
    LessThan
}

#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub enum DfMirCall {
    Defined(u128),
    Ptr(Temporary),
    Extern(String),
    Intrinsic(DfMirCallIntrinsic)
}
impl Debug for DfMirCall { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    match (self) {
        Self::Defined(func_id) => { write!(f, "{}", func_id)?; },
        Self::Ptr(ptr)         => { write!(f, "*{:?}", ptr)?; },
        Self::Extern(name)     => { write!(f, "extern.{}", name)?; },
        Self::Intrinsic(name)  => { write!(f, "intrinsic.{:?}", name)?; }
    }
    Ok(())
} }

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
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
