use core::hash::{ Hash, Hasher };
use std::hash::DefaultHasher;
use rustc_errors::{ Diag, Level, ErrorGuaranteed };
use rustc_middle::{
    middle::codegen_fn_attrs::CodegenFnAttrFlags,
    mir::{
        Body,
        BasicBlockData,
        Statement,
        StatementKind,
        Terminator,
        TerminatorKind
    },
    ty::{
        Instance,
        TyCtxt
    }
};
use rustc_span::def_id::DefId;


pub fn mangle_name(tcx : TyCtxt<'_>, def_id : DefId) -> String {
    let attrs = tcx.body_codegen_attrs(def_id);
    if let Some(name) = attrs.symbol_name {
        name.to_string()
    } else if (attrs.flags.contains(CodegenFnAttrFlags::NO_MANGLE)) {
        tcx.item_name(def_id).to_string()
    } else {
        let mut hasher = DefaultHasher::new();
        def_id.hash(&mut hasher);
        format!("{}__{:0>16x}", tcx.item_name(def_id), hasher.finish())
    }
}


pub fn mir_to_dfmir<'tcx>(
    tcx      : TyCtxt<'tcx>,
    instance : Instance<'tcx>,
    mir      : &Body<'tcx>
) {
    let name = mangle_name(tcx, instance.def_id());
    if (name == "rust_begin_unwind") { return; }

    // println!("  {}:", name);
    for (i, bb,) in mir.basic_blocks.iter().enumerate() {
        // println!("    bb{}:", i);
        bb_to_dfmir(tcx, instance, bb);
    }
}


fn bb_to_dfmir<'tcx>(
    tcx      : TyCtxt<'tcx>,
    instance : Instance<'tcx>,
    bb       : &BasicBlockData<'tcx>
) {
    for stmt in &bb.statements {
        stmt_to_dfmir(tcx, stmt);
    }
    if let Some(term) = &bb.terminator {
        term_to_dfmir(tcx, term);
    }
}


fn stmt_to_dfmir<'tcx>(
    tcx  : TyCtxt<'tcx>,
    stmt : &Statement<'tcx>
) {
    match (stmt.kind) {

        StatementKind::Assign(_) => { /* TODO */ },

        StatementKind::FakeRead(_) => { unreachable!("disallowed after drop elaboration") },

        StatementKind::SetDiscriminant { .. } => { /* TODO */ },

        StatementKind::StorageLive(_) => { /* TODO */ },

        StatementKind::StorageDead(_) => { /* TODO */ },

        StatementKind::Retag(_, _) => { /* TODO */ },

        StatementKind::PlaceMention(_) => { /* TODO */ },

        StatementKind::AscribeUserType(_, _) => { /* TODO */ },

        StatementKind::Coverage(_) => { /* TODO */ },

        StatementKind::Intrinsic(_) => { /* TODO */ },

        StatementKind::ConstEvalCounter => { },

        StatementKind::Nop => { },

        StatementKind::BackwardIncompatibleDropHint { .. } => { }

    }
}


fn term_to_dfmir<'tcx>(
    tcx  : TyCtxt<'tcx>,
    term : &Terminator<'tcx>
) {
    match (term.kind) {

        TerminatorKind::Goto { .. } => { /* TODO */ },

        TerminatorKind::SwitchInt { .. } => { /* TODO */ },

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

        TerminatorKind::Return => { /* TODO */ },

        TerminatorKind::Unreachable => { },

        TerminatorKind::Drop { .. } => { /* TODO */ },

        TerminatorKind::Call { .. } => { /* TODO */ },

        TerminatorKind::TailCall { .. } => { /* TODO */ },

        TerminatorKind::Assert { .. } => { /* TODO */ },

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
