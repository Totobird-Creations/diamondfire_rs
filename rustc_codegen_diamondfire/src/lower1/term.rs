use crate::{
    HashingUtil,
    diag
};
use super::Lower1Ctx;
use bridgecg_diamondfire::dfmir::DfMirStmt;
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

        TerminatorKind::Drop { .. } => {
            // TODO
        },

        TerminatorKind::Call { func, args, destination, target, unwind, .. } => {
            if let UnwindAction::Unreachable = unwind { } else {
                diag::unwinding_unsupported(ctx.tcx.dcx(), term.source_info.span);
            }
            let func_ty = func.ty(ctx.body, ctx.tcx);
            match (func_ty.kind()) {
                TyKind::FnDef(def_id, genargs) => {
                    let fn_id = HashingUtil::hash_fn_def(ctx.tcx, *def_id, *genargs);
                    if let Some(intrinsic) = ctx.tcx.intrinsic(*def_id) {
                        match (intrinsic.name.as_str()) {
                            name => { diag::intrinsic_unsupported(ctx.tcx.dcx(), term.source_info.span, name); }
                        }
                    } else if (ctx.tcx.is_foreign_item(*def_id)) {
                        out.push(DfMirStmt::CallExtern {
                            name : ctx.tcx.codegen_fn_attrs(*def_id).symbol_name.unwrap_or_else(|| ctx.tcx.item_name(*def_id)).to_string()
                            // TODO
                        });
                    } else {
                        out.push(DfMirStmt::Call {
                            name  : ctx.tcx.codegen_fn_attrs(*def_id).symbol_name.unwrap_or_else(|| ctx.tcx.item_name(*def_id)).to_string(),
                            fn_id
                            // TODO
                        });
                    }
                },
                TyKind::FnPtr(_, _) => {
                    out.push(DfMirStmt::CallPtr { // TODO
                    });
                },
                tyk => unreachable!("{:?} {:?}", std::mem::discriminant(tyk), tyk)
            }
        },

        TerminatorKind::TailCall { .. } => todo!(),

        TerminatorKind::Assert { .. } => {
            // TODO
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
