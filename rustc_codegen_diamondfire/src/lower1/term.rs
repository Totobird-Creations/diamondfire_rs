use crate::{
    HashingUtil,
    diag
};
use super::{
    Lower1Ctx,
    place_load_to_dfmir,
    place_store_to_dfmir,
    operand_to_dfmir,
    drop_to_dfmir
};
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirCall,
    DfMirCallIntrinsic
};
use rustc_middle::{
    mir::{
        Terminator,
        TerminatorKind,
        UnwindAction
    },
    ty::TyKind
};


pub fn term_to_dfmir<'tcx>(
    ctx  : &mut Lower1Ctx<'tcx, '_>,
    term : &Terminator<'tcx>,
    out  : &mut Vec<DfMirStmt>
) {
    match (&term.kind) {

        TerminatorKind::Goto { .. }
        | TerminatorKind::SwitchInt { .. }
        | TerminatorKind::Return
        | TerminatorKind::Unreachable
        => { },

        TerminatorKind::Drop { place, unwind, drop, async_fut, .. } => {
            if let UnwindAction::Unreachable = unwind { } else {
                diag::unwinding_unsupported(ctx.tcx.dcx(), term.source_info.span);
            }
            if (drop.is_some() || async_fut.is_some()) {
                diag::coroutines_unsupported(ctx.tcx.dcx(), term.source_info.span);
            }
            let (place_df, place_ty,) = place_load_to_dfmir(ctx, place, out);
            drop_to_dfmir(ctx, place_ty, place_df, out);
        },

        TerminatorKind::Call { func, args, destination, unwind, .. } => {
            if let UnwindAction::Unreachable = unwind { } else {
                diag::unwinding_unsupported(ctx.tcx.dcx(), term.source_info.span);
            }
            let func_ty = func.ty(ctx.body, ctx.tcx);
            let call = { match (func_ty.kind()) {
                TyKind::FnDef(def_id, generics) => {
                    if let Some(intrinsic) = ctx.tcx.intrinsic(*def_id) {
                        DfMirCall::Intrinsic(match (intrinsic.name.as_str()) {
                            "abort"                    => DfMirCallIntrinsic::Abort,
                            "arith_offset"             => DfMirCallIntrinsic::PtrShift,
                            "assert_inhabited"         => { return; },
                            "bswap"                    => DfMirCallIntrinsic::ByteReverse,
                            "caller_location"          => DfMirCallIntrinsic::CallerLocation,
                            "ceilf32"                  => DfMirCallIntrinsic::CeilF32,
                            "ceilf64"                  => DfMirCallIntrinsic::CeilF64,
                            "cold_path"                => { return; },
                            "compare_bytes"            => DfMirCallIntrinsic::CompareBytes,
                            "copysignf64"              => DfMirCallIntrinsic::CopySignF64,
                            "ctlz"                     => DfMirCallIntrinsic::CountLeadingZeroBits,
                            "ctpop"                    => DfMirCallIntrinsic::CountOneBits,
                            "cttz"|"cttz_nonzero"      => DfMirCallIntrinsic::CountTrailingZeroBits,
                            "disjoint_bitor"           => DfMirCallIntrinsic::DisjointBitOr,
                            "fabsf32"                  => DfMirCallIntrinsic::AbsF32,
                            "fabsf64"                  => DfMirCallIntrinsic::AbsF64,
                            "floorf32"                 => DfMirCallIntrinsic::FloorF32,
                            "floorf64"                 => DfMirCallIntrinsic::FloorF64,
                            "fmaf64"                   => DfMirCallIntrinsic::MulAddF64,
                            "ptr_offset_from_unsigned" => DfMirCallIntrinsic::PtrOffsetFromUnsigned,
                            "round_ties_even_f32"      => DfMirCallIntrinsic::RoundTiesEvenF32,
                            "round_ties_even_f64"      => DfMirCallIntrinsic::RoundTiesEvenF64,
                            "saturating_sub"           => DfMirCallIntrinsic::SaturatingSub,
                            "select_unpredictable"     => DfMirCallIntrinsic::SelectUnpredictable,
                            "sqrtf32"                  => DfMirCallIntrinsic::SqrtF32,
                            "sqrtf64"                  => DfMirCallIntrinsic::SqrtF64,
                            "truncf32"                 => DfMirCallIntrinsic::TruncF32,
                            "truncf64"                 => DfMirCallIntrinsic::TruncF64,
                            "unchecked_funnel_shl"     => DfMirCallIntrinsic::FunnelShlUnchecked,
                            name => {
                                diag::intrinsic_unsupported(ctx.tcx.dcx(), term.source_info.span, name);
                                return;
                            }
                        })
                    } else if (ctx.tcx.is_foreign_item(*def_id)) {
                        DfMirCall::Extern(ctx.tcx.codegen_fn_attrs(*def_id).symbol_name.unwrap_or_else(|| ctx.tcx.item_name(*def_id)).to_string())
                    } else {
                        DfMirCall::Defined(HashingUtil::hash_fn_def(ctx.tcx, *def_id, *generics))
                    }
                },
                TyKind::FnPtr(_, _) => {
                    let func_df = operand_to_dfmir(ctx, func, out);
                    DfMirCall::Ptr(func_df)
                },
                tyk => unreachable!("{:?} {:?}", std::mem::discriminant(tyk), tyk)
            } };
            let args = args.iter().map(|arg| operand_to_dfmir(ctx, &arg.node, out)).collect::<Vec<_>>();
            let dst  = ctx.next_temp();
            out.push(DfMirStmt::Call { dst, call, args });
            place_store_to_dfmir(ctx, destination, dst, out)
        },

        TerminatorKind::TailCall { .. } => todo!(),

        TerminatorKind::Assert { cond, expected, msg, target, unwind } => {
            if let UnwindAction::Unreachable = unwind { } else {
                diag::unwinding_unsupported(ctx.tcx.dcx(), term.source_info.span);
            }
            todo!()
        },

        TerminatorKind::UnwindResume
        | TerminatorKind::UnwindTerminate(_)
        => { diag::unwinding_unsupported(ctx.tcx.dcx(), term.source_info.span); },

        TerminatorKind::Yield { .. }
        | TerminatorKind::CoroutineDrop
        => { diag::coroutines_unsupported(ctx.tcx.dcx(), term.source_info.span); },

        TerminatorKind::FalseEdge { .. }
        | TerminatorKind::FalseUnwind { .. }
        => { diag::disallowed_post_drop_elaboration(); },

        TerminatorKind::InlineAsm { .. }
        => { diag::inlineasm_unsupported(ctx.tcx.dcx(), term.source_info.span); }

    }
}
