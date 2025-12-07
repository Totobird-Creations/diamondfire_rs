use crate::{
    dfmir::{
        DfMirFn,
        DfMirStmt
    },
    diag
};
use super::operand_to_dfmir;
use rustc_middle::{
    mir::{
        Terminator,
        TerminatorKind,
        UnwindAction
    },
    ty::TyCtxt
};


pub fn term_to_dfmir<'tcx>(
    dest : &mut DfMirFn,
    tcx  : TyCtxt<'tcx>,
    term : &Terminator<'tcx>
) {
    match (&term.kind) {

        TerminatorKind::Goto { .. } => {
            todo!();
        },

        TerminatorKind::SwitchInt { discr, targets } => {
            let df_discr = operand_to_dfmir(dest, tcx, discr);
            todo!();
        },

        TerminatorKind::UnwindResume
        | TerminatorKind::UnwindTerminate(_)
        => { /* Error emitted in CFG recovery */ },

        TerminatorKind::Return => {
            dest.push_stmt(DfMirStmt::Return);
        },

        TerminatorKind::Unreachable => {
            todo!()
        },

        TerminatorKind::Drop { unwind, drop, async_fut, .. } => {
            if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                diag::unwinding_unsupported(tcx.dcx(), term.source_info.span);
            }
            if (drop.is_some() || async_fut.is_some()) {
                diag::coroutines_unsupported(tcx.dcx(), term.source_info.span);
            }
            todo!();
        },

        TerminatorKind::Call { unwind, .. } => {
            if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                diag::unwinding_unsupported(tcx.dcx(), term.source_info.span);
            }
            todo!();
        },

        TerminatorKind::TailCall { .. } => {
            todo!();
        },

        TerminatorKind::Assert { unwind, .. } => {
            if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                diag::unwinding_unsupported(tcx.dcx(), term.source_info.span);
            }
            todo!();
        },

        TerminatorKind::Yield { .. }
        | TerminatorKind::CoroutineDrop
        => { /* Error emitted in CFG recovery */ },

        TerminatorKind::FalseEdge { .. }
        | TerminatorKind::FalseUnwind { .. }
        => { /* Panic emitted in CFG recovery */ },

        TerminatorKind::InlineAsm { .. } => { /* Error emitted in CFG recovery */ }

    }
}
