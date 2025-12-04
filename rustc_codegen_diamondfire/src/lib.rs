#![feature(rustc_private)]


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
    config::{
        OutputFilenames,
        OutputType
    },
    Session
};


mod cfg;
mod dfmir;

mod lower1;


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

        let module_items = tcx.hir_crate_items(());
        for item_id in module_items.free_items() {
            let item = tcx.hir_item(item_id);
            match (item.kind) {
                ItemKind::ExternCrate(_, _,) => { },
                ItemKind::Use(_, _,) => { },
                ItemKind::Static(_, _, _, _,) => {
                    // TODO
                },
                ItemKind::Const(_, _, _, _,) => { },
                ItemKind::Fn { .. } => {
                    let def_id   = item_id.owner_id.to_def_id();
                    let generics = tcx.generics_of(def_id);
                    if (! generics.own_params.is_empty()) {
                        // Skipped direct lowering of generic function.
                        continue;
                    }
                    let instance = Instance::mono(tcx, def_id);
                    let mir      = tcx.optimized_mir(def_id);
                    lower1::mir_to_dfmir(tcx, instance, mir);
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
        sess            : &Session,
        outputs         : &OutputFilenames
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>,) {
        let ongoing_codegen = ongoing_codegen.downcast::<CrateToJoin>().unwrap();

        // TODO: Write data used by the linker.
        println!("\n{:?}", ongoing_codegen.crate_info.crate_types);
        println!("{}", ongoing_codegen.crate_info.local_crate_name);
        println!("{:?}\n", outputs.with_extension("dfrs-cg"));
        let mut f = File::create(outputs.with_extension("dfrs-cg")).unwrap();

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
