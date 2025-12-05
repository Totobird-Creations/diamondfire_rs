use crate::{
    dfmir::{
        DfMirFn,
        DfMirStmt
    },
    diag
};
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

        TerminatorKind::SwitchInt { .. } => {
            todo!();
        },

        TerminatorKind::UnwindResume { .. } => { diag::unwinding_unsupported(tcx.dcx(), term.source_info.span); },

        TerminatorKind::UnwindTerminate(_) => { diag::unwinding_unsupported(tcx.dcx(), term.source_info.span); },

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

        TerminatorKind::Yield { .. } => { diag::coroutines_unsupported(tcx.dcx(), term.source_info.span); },

        TerminatorKind::CoroutineDrop => { diag::coroutines_unsupported(tcx.dcx(), term.source_info.span); },

        TerminatorKind::FalseEdge { .. } => { unreachable!("disallowed after drop elaboration") },

        TerminatorKind::FalseUnwind { .. } => { unreachable!("disallowed after drop elaboration") },

        TerminatorKind::InlineAsm { .. } => { diag::inlineasm_unsupported(tcx.dcx(), term.source_info.span); }

    }
}
