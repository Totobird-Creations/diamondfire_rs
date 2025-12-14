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
    ctx  : &mut Lower1Ctx<'tcx>,
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
                    out.push(DfMirStmt::Call {
                        fn_id
                    });
                },
                tyk => unreachable!("{:?}", tyk)
            }
        },

        TerminatorKind::TailCall { .. } => todo!(),

        TerminatorKind::Assert { .. } => todo!(),

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
