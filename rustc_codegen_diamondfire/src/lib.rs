#![feature(
    rustc_private,
    f128,
    debug_closure_helpers,
    map_try_insert
)]


extern crate rustc_codegen_ssa;
extern crate rustc_codegen_llvm;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_query_system;
extern crate rustc_session;
extern crate rustc_span;

use core::any::Any;
use std::{
    fs::File,
    path::Path
};
use rustc_codegen_ssa::{
    back::{
        archive::{
            ArArchiveBuilder,
            ArchiveBuilder,
            ArchiveBuilderBuilder,
            ImportLibraryItem,
            DEFAULT_OBJECT_READER
        },
        link::link_binary
    },
    traits::CodegenBackend,
    CodegenResults,
    CrateInfo
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::ItemKind;
use rustc_metadata::EncodedMetadata;
use rustc_middle::ty::{
    Instance,
    TyCtxt
};
use rustc_query_system::dep_graph::{
    dep_node::WorkProductId,
    WorkProduct
};
use rustc_session::{
    config::OutputFilenames,
    Session
};


pub mod cfg;
pub mod dfmir;

pub mod lower1;

pub mod diag;


struct CrateToJoin {
    crate_info : CrateInfo
}


pub struct DiamondfireCodegen;

impl CodegenBackend for DiamondfireCodegen {

    fn name(&self) -> &'static str { "diamondfire" }

    fn locale_resource(&self) -> &'static str { "" }


    fn codegen_crate<'tcx>(&self, tcx : TyCtxt<'tcx>) -> Box<dyn Any> {
        let crate_info = CrateInfo::new(tcx, "diamondfire".to_string());
        let crate_name = crate_info.local_crate_name.to_string();
        if (crate_name == "core" || crate_name == "compiler_builtins" || crate_name == "diamondfire" || crate_name == "diamondfire-sys") { // TODO: Remove
            return Box::new(CrateToJoin { crate_info });
        }

        for item_id in tcx.hir_crate_items(()).free_items() {
            let item = tcx.hir_item(item_id);
            match (item.kind) {
                ItemKind::ExternCrate(_, _,) => { },
                ItemKind::Use(_, _,) => { },
                ItemKind::Static(_, _, _, _,) => {
                    // TODO
                },
                ItemKind::Const(_, _, _, _,) => { },
                ItemKind::Fn { body, .. } => {
                    let def_id   = item_id.owner_id.to_def_id();
                    let generics = tcx.generics_of(def_id);
                    if (! generics.own_params.is_empty()) {
                        todo!();
                    }
                    let instance = Instance::mono(tcx, def_id);
                    let hir      = tcx.hir_body(body);
                    let mir      = tcx.optimized_mir(def_id);
                    // for scope in mir.source_scopes.iter() { if let Some((inlined, _,)) = &scope.inlined {
                    //     rustc_hir::BodyId { hir_id : rustc_hir::HirId { owner : inlined.def_id(). } }
                    //     // println!("{:?}", tcx.hir_body());
                    // } }
                    println!();
                    for (bbi, bb,) in mir.basic_blocks.iter().enumerate() {
                        println!("bb{:?}:", bbi);
                        for stmt in &bb.statements {
                            println!("  {:?}", stmt);
                        }
                        println!("  {:?}", bb.terminator().kind);
                    }
                    println!();
                    let branches  = cfg::find_body_cfb(&tcx, hir);
                    let cfa_prims = cfg::analyse_cfb(tcx, branches, &mir.basic_blocks);
                    println!("{:#?}", cfa_prims);
                    println!();
                    let cfg_tree  = cfg::recover_cfg(&cfa_prims);
                    println!("{:#}", cfg_tree);
                    // lower1::mir_to_dfmir(tcx, instance, mir);
                },
                ItemKind::Macro(_, _, _,) => { },
                ItemKind::Mod(_, _,) => { },
                ItemKind::ForeignMod { .. } => { },
                ItemKind::GlobalAsm { .. } => {
                    // TODO
                },
                ItemKind::TyAlias(_, _, _,) => { },
                ItemKind::Enum(_, _, _,) => { },
                ItemKind::Struct(_, _, _,) => { },
                ItemKind::Union(_, _, _,) => { },
                ItemKind::Trait(_, _, _, _, _, _, _,) => { },
                ItemKind::TraitAlias(_, _, _, _,) => { },
                ItemKind::Impl(_,) => {
                    // TODO
                }
            }
        }
        Box::new(CrateToJoin {
            crate_info,
        })
    }


    fn join_codegen(&self,
        ongoing_codegen : Box<dyn Any>,
        _sess           : &Session,
        outputs         : &OutputFilenames
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>,) {
        let ongoing_codegen = ongoing_codegen.downcast::<CrateToJoin>().unwrap();

        println!("\n{:?}", ongoing_codegen.crate_info.crate_types);
        println!("{}", ongoing_codegen.crate_info.local_crate_name);
        println!("{:?}\n", outputs.with_extension("dfrs-cg"));
        File::create(outputs.with_extension("dfrs-cg")).unwrap();
        // TODO: Write data used by the linker.

        (CodegenResults {
            modules          : Vec::new(),
            allocator_module : None,
            crate_info       : ongoing_codegen.crate_info
        }, FxIndexMap::default(),)
    }


    fn link(&self,
        sess            : &Session,
        codegen_results : CodegenResults,
        metadata        : EncodedMetadata,
        outputs         : &OutputFilenames
    ) {
        link_binary(
            sess,
            &RlibArchiveBuilder,
            codegen_results,
            metadata,
            outputs,
            "diamondfire"
        );
    }


}


struct RlibArchiveBuilder;

impl ArchiveBuilderBuilder for RlibArchiveBuilder {

    fn new_archive_builder<'a>(&self, sess: &'a Session) -> Box<dyn ArchiveBuilder + 'a> {
        Box::new(ArArchiveBuilder::new(
            sess,
            &DEFAULT_OBJECT_READER,
        ))
    }

    fn create_dll_import_lib(
        &self,
        _sess        : &Session,
        _lib_name    : &str,
        _dll_imports : Vec<ImportLibraryItem>,
        _tmpdir      : &Path,
    ) { unimplemented!("creating dll imports is unsupported"); }

}


#[unsafe(no_mangle)]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(DiamondfireCodegen)
}
