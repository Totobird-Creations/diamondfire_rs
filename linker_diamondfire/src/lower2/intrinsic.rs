use crate::ctx::LinkingCtx;
use bridgecg_diamondfire::{
    dfmir::DfMirCallIntrinsic,
    dflir::{
        DfLirBlock,
        DfLirTarget
    },
    Temporary
};


pub fn intrinsic_call_to_dflir(ctx : &mut LinkingCtx, dst : Temporary, intrinsic : DfMirCallIntrinsic, args : &[Temporary], out : &mut Vec<DfLirBlock<'static>>) {
    match (intrinsic) {

        DfMirCallIntrinsic::Abort => {
            out.extend([
                // TODO: Send abort message.
                DfLirBlock::PlayerAction {
                    action : "Kick",
                    target : DfLirTarget::AllPlayers,
                    args   : Vec::new()
                },
                DfLirBlock::Control {
                    action : "End",
                    args   : Vec::new()
                }
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
