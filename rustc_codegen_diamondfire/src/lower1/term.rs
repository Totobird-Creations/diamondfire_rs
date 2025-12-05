use rustc_errors::{ Diag, Level, ErrorGuaranteed };
use rustc_middle::{
    mir::{
        Terminator,
        TerminatorKind,
        UnwindAction
    },
    ty::TyCtxt
};


pub fn term_to_dfmir<'tcx>(
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

        TerminatorKind::UnwindResume { .. } => {
            Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "unwinding is currently unsupported by the `diamondfire-unknown-unknown` target")
                .with_span(term.source_info.span)
                .emit();
        },

        TerminatorKind::UnwindTerminate(_) => {
            Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "unwinding is currently unsupported by the `diamondfire-unknown-unknown` target")
                .with_span(term.source_info.span)
                .emit();
        },

        TerminatorKind::Return => {
            println!("      return");
        },

        TerminatorKind::Unreachable => {
            println!("      unreachable");
        },

        TerminatorKind::Drop { unwind, drop, async_fut, .. } => {
            if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "unwinding is currently unsupported by the `diamondfire-unknown-unknown` target")
                    .with_span(term.source_info.span)
                    .emit();

            }
            if (drop.is_some() || async_fut.is_some()) {
                Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "coroutines are currently unsupported by the `diamondfire-unknown-unknown` target")
                    .with_span(term.source_info.span)
                    .emit();
            }
            todo!();
        },

        TerminatorKind::Call { unwind, .. } => {
            if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "unwinding is currently unsupported by the `diamondfire-unknown-unknown` target")
                    .with_span(term.source_info.span)
                    .emit();
            }
            todo!();
        },

        TerminatorKind::TailCall { .. } => {
            todo!();
        },

        TerminatorKind::Assert { unwind, .. } => {
            if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "unwinding is currently unsupported by the `diamondfire-unknown-unknown` target")
                    .with_span(term.source_info.span)
                    .emit();
            }
            todo!();
        },

        TerminatorKind::Yield { .. } => {
            Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "coroutines are currently unsupported by the `diamondfire-unknown-unknown` target")
                .with_span(term.source_info.span)
                .emit();
        },

        TerminatorKind::CoroutineDrop => {
            Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "coroutines are currently unsupported by the `diamondfire-unknown-unknown` target")
                .with_span(term.source_info.span)
                .emit();
        },

        TerminatorKind::FalseEdge { .. } => { unreachable!("disallowed after drop elaboration") },

        TerminatorKind::FalseUnwind { .. } => { unreachable!("disallowed after drop elaboration") },

        TerminatorKind::InlineAsm { .. } => {
            Diag::<ErrorGuaranteed>::new(tcx.dcx(), Level::Error, "inline assembly is unsupported by the `diamondfire-unknown-unknown` target")
                .with_span(term.source_info.span)
                .emit();
        }

    }
}
