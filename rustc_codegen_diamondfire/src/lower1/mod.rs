use crate::dfmir::DfMirFn;
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
        TyCtxt,
        TyKind
    }
};
use rustc_hir::def_id::{
    DefId,
    LOCAL_CRATE
};


mod stmt;
pub use stmt::*;
mod term;
pub use term::*;
mod place;
pub use place::*;
mod rvalue;
pub use rvalue::*;
mod operand;
pub use operand::*;
mod ty;
pub use ty::*;


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

    let mut dest = DfMirFn::new(name);

    let instance_ty      = tcx.type_of(instance.def_id()).instantiate(tcx, instance.args);
    let instance_ty_kind = instance_ty.kind();
    match (instance_ty_kind) {
        TyKind::FnDef(_, _) => {
            let sig = instance_ty.fn_sig(tcx);
            for input in sig.inputs().skip_binder() {
                _ = dest.push_param(ty_to_dfmir(tcx, input));
            }
            dest.insert_local(0, ty_to_dfmir(tcx, &sig.output().skip_binder()));
        },
        _ => todo!("{:?}", instance_ty_kind)
    }

    for bb in mir.basic_blocks.iter() {
        dest.push_block();
        bb_to_dfmir(&mut dest, tcx, instance, bb);
    }

    println!("{:#?}", dest);
}


fn bb_to_dfmir<'tcx>(
    dest      : &mut DfMirFn,
    tcx       : TyCtxt<'tcx>,
    _instance : Instance<'tcx>,
    bb        : &BasicBlockData<'tcx>
) {
    for stmt in &bb.statements {
        stmt_to_dfmir(dest, tcx, stmt);
    }
    if let Some(term) = &bb.terminator {
        term_to_dfmir(dest, tcx, term);
    }
}
