//! DiamondFire Mid-Level Intermediate Representation
//!
//! dfMIR is quite similar to rsMIR but with different statement types.

use core::fmt::{ self, Debug, Formatter };
use bincode::{ Encode, Decode };

mod ty;
pub use ty::*;


#[derive(Encode, Decode, Debug)]
pub enum DfMirStmt {

    CopyTL {
        src : DfMirTemporary,
        dst : DfMirLocal
    },
    CopyLT {
        src : DfMirLocal,
        dst : DfMirTemporary
    },
    CopyTT {
        src : DfMirTemporary,
        dst : DfMirTemporary
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

    Call {
        call : DfMirCall,
        // TODO: Params
        // TODO: Returns
    },

    Return

}

#[derive(Encode, Decode, Debug)]
pub enum DfMirCall {
    Defined(u128),
    Ptr,
    Extern(String),
    Intrinsic(DfMirCallIntrinsic)
}

#[derive(Clone, Copy, Encode, Decode)]
pub struct DfMirLocal(pub usize);
impl Debug for DfMirLocal { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    write!(f, "local.{}", self.0)
} }

#[derive(Clone, Copy, Encode, Decode)]
pub struct DfMirTemporary(pub usize);
impl DfMirTemporary {
    pub const PLACEHOLDER : Self = Self(usize::MAX);
}
impl Debug for DfMirTemporary { fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
    write!(f, "temp.{}", self.0)
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

    /// Approximates the total number of dfJSON code blocks that several statements will become.
    ///
    /// Used to determine if a function is applicable for inlining.
    pub fn dfjson_cost_many(stmts : impl IntoIterator<Item = DfMirStmt>) -> usize {
        stmts.into_iter().map(|stmt| stmt.dfjson_cost()).sum()
    }

    /// Approximates the number of dfJSON code blocks this statement will become.
    pub fn dfjson_cost(&self) -> usize {
        match (self) {
            Self::CopyTL { .. }          => 1,
            Self::CopyLT { .. }          => 1,
            Self::CopyTT { .. }          => 1,
            Self::TNumber { .. }         => 1,
            Self::TStruct { fields, .. } => fields.len().div_ceil(26) + 1,
            Self::TEnum { fields, .. }   => (fields.len() + 1).div_ceil(26) + 1,
            Self::Call { call, .. }         => { match (call) {
                DfMirCall::Defined(_) => 1,
                DfMirCall::Ptr        => 1,
                DfMirCall::Extern(_)  => 1,
                DfMirCall::Intrinsic(intrinsic) => { match (intrinsic) {
                    DfMirCallIntrinsic::Abort                 => todo!(),
                    DfMirCallIntrinsic::AbsF32                => todo!(),
                    DfMirCallIntrinsic::AbsF64                => todo!(),
                    DfMirCallIntrinsic::ByteReverse           => todo!(),
                    DfMirCallIntrinsic::CallerLocation        => todo!(),
                    DfMirCallIntrinsic::CeilF32               => todo!(),
                    DfMirCallIntrinsic::CeilF64               => todo!(),
                    DfMirCallIntrinsic::CompareBytes          => todo!(),
                    DfMirCallIntrinsic::CopySignF64           => todo!(),
                    DfMirCallIntrinsic::CountLeadingZeroBits  => todo!(),
                    DfMirCallIntrinsic::CountOneBits          => todo!(),
                    DfMirCallIntrinsic::CountTrailingZeroBits => todo!(),
                    DfMirCallIntrinsic::FloorF32              => todo!(),
                    DfMirCallIntrinsic::FloorF64              => todo!(),
                    DfMirCallIntrinsic::FunnelShlUnchecked    => todo!(),
                    DfMirCallIntrinsic::DisjointBitOr         => todo!(),
                    DfMirCallIntrinsic::MulAddF64             => todo!(),
                    DfMirCallIntrinsic::PtrShift              => todo!(),
                    DfMirCallIntrinsic::PtrOffsetFromUnsigned => todo!(),
                    DfMirCallIntrinsic::RoundTiesEvenF32      => todo!(),
                    DfMirCallIntrinsic::RoundTiesEvenF64      => todo!(),
                    DfMirCallIntrinsic::SaturatingSub         => todo!(),
                    DfMirCallIntrinsic::SelectUnpredictable   => todo!(),
                    DfMirCallIntrinsic::SqrtF32               => todo!(),
                    DfMirCallIntrinsic::SqrtF64               => todo!(),
                    DfMirCallIntrinsic::TruncF32              => todo!(),
                    DfMirCallIntrinsic::TruncF64              => todo!()
                } }
            } },
            Self::Return => 1,
        }
    }

}
