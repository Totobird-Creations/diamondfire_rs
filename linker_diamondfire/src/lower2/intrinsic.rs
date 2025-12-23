use crate::ctx::LinkingCtx;
use bridgecg_diamondfire::{
    dfmir::{
        DfMirCallIntrinsic,
        DfMirTemporary
    },
    dflir::{
        DfLirBlock,
        DfLirTarget
    }
};


pub fn intrinsic_call_to_dflir(ctx : &mut LinkingCtx, dst : DfMirTemporary, intrinsic : DfMirCallIntrinsic, args : &[DfMirTemporary], out : &mut Vec<DfLirBlock<'static>>) {
    match (intrinsic) {

        DfMirCallIntrinsic::Abort => {
            out.extend([
                // TODO: Send abort message.
                DfLirBlock::PlayerAction { action : "Kick", target : DfLirTarget::AllPlayers },
                DfLirBlock::Control { action : "End" }
            ]);
        },

        DfMirCallIntrinsic::AbsF32 => todo!(),

        DfMirCallIntrinsic::AbsF64 => todo!(),

        DfMirCallIntrinsic::ByteReverse => todo!(),

        DfMirCallIntrinsic::CallerLocation => todo!(),

        DfMirCallIntrinsic::CeilF32 => todo!(),

        DfMirCallIntrinsic::CeilF64 => todo!(),

        DfMirCallIntrinsic::CompareBytes => todo!(),

        DfMirCallIntrinsic::CopySignF64 => todo!(),

        DfMirCallIntrinsic::CountLeadingZeroBits => todo!(),

        DfMirCallIntrinsic::CountOneBits => todo!(),

        DfMirCallIntrinsic::CountTrailingZeroBits => todo!(),

        DfMirCallIntrinsic::FloorF32 => todo!(),

        DfMirCallIntrinsic::FloorF64 => todo!(),

        DfMirCallIntrinsic::FunnelShlUnchecked => todo!(),

        DfMirCallIntrinsic::DisjointBitOr => todo!(),

        DfMirCallIntrinsic::MulAddF64 => todo!(),

        DfMirCallIntrinsic::PtrShift => todo!(),

        DfMirCallIntrinsic::PtrOffsetFromUnsigned => todo!(),

        DfMirCallIntrinsic::RoundTiesEvenF32 => todo!(),

        DfMirCallIntrinsic::RoundTiesEvenF64 => todo!(),

        DfMirCallIntrinsic::SaturatingSub => todo!(),

        DfMirCallIntrinsic::SelectUnpredictable => todo!(),

        DfMirCallIntrinsic::SqrtF32 => todo!(),

        DfMirCallIntrinsic::SqrtF64 => todo!(),

        DfMirCallIntrinsic::TruncF32 => todo!(),

        DfMirCallIntrinsic::TruncF64 => todo!()

    }
}
