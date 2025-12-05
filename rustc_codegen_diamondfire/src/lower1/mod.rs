use core::hash::{ Hash, Hasher };
use std::hash::DefaultHasher;
use rustc_middle::{
    middle::codegen_fn_attrs::CodegenFnAttrFlags,
    mir::{
        Body,
        BasicBlockData
    },
    ty::{
        Instance,
        TyCtxt
    }
};
use rustc_hir::def_id::{
    DefId,
    LOCAL_CRATE
};


mod stmt;
pub use stmt::stmt_to_dfmir;
mod term;
pub use term::term_to_dfmir;
mod place;
pub use place::place_read_to_dfmir;
mod rvalue;
pub use rvalue::rvalue_to_dfmir;
mod operand;
pub use operand::operand_to_dfmir;
mod r#const;
pub use r#const::{
    const_to_dfmir,
    tyconst_to_dfmir
};


pub fn mangle_name(tcx : TyCtxt<'_>, def_id : DefId) -> String {
    let attrs = tcx.body_codegen_attrs(def_id);
    if let Some(name) = attrs.symbol_name {
        name.to_string()
    } else if (attrs.flags.contains(CodegenFnAttrFlags::NO_MANGLE)) {
        tcx.item_name(def_id).to_string()
    } else {
        let mut hasher = DefaultHasher::new();
        def_id.hash(&mut hasher);
        format!("{}__{}__{:0>16x}", tcx.crate_name(LOCAL_CRATE), tcx.item_name(def_id), hasher.finish())
    }
}


pub fn mir_to_dfmir<'tcx>(
    tcx      : TyCtxt<'tcx>,
    instance : Instance<'tcx>,
    mir      : &Body<'tcx>
) {
    let name = mangle_name(tcx, instance.def_id());
    if (name == "rust_begin_unwind") { return; }

    println!("  {}:", name);
    for (i, bb,) in mir.basic_blocks.iter().enumerate() {
        println!("    bb{}:", i);
        bb_to_dfmir(tcx, instance, bb);
    }
}


fn bb_to_dfmir<'tcx>(
    tcx       : TyCtxt<'tcx>,
    _instance : Instance<'tcx>,
    bb        : &BasicBlockData<'tcx>
) {
    for stmt in &bb.statements {
        stmt_to_dfmir(tcx, stmt);
    }
    if let Some(term) = &bb.terminator {
        term_to_dfmir(tcx, term);
    }
}
