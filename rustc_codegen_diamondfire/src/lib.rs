#![feature(
    rustc_private,
    f128,
    debug_closure_helpers,
    map_try_insert,
    assert_matches
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
    CompiledModule,
    ModuleKind,
    CrateInfo
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    mir::mono::MonoItem,
    ty::TyCtxt,
};
use rustc_query_system::dep_graph::{
    dep_node::WorkProductId,
    WorkProduct
};
use rustc_session::{
    config::OutputFilenames,
    Session
};
use rustc_span::DUMMY_SP;


pub mod cfr;
pub mod dfmir;

// pub mod lower1;

pub mod diag;


struct CrateToJoin {
    crate_info : CrateInfo
}


pub struct DiamondfireCodegen;

impl CodegenBackend for DiamondfireCodegen {

    fn name(&self) -> &'static str { "diamondfire" }

    fn locale_resource(&self) -> &'static str { "" }


    fn codegen_crate<'tcx>(&self, tcx : TyCtxt<'tcx>) -> Box<dyn Any> {
        let mut crate_info = CrateInfo::new(tcx, "diamondfire".to_string());

        let crate_name = crate_info.local_crate_name.to_string();
        if (crate_name == "compiler_builtins") { // TODO: Remove
            return Box::new(CrateToJoin { crate_info });
        }

        for codegen_unit in tcx.collect_and_partition_mono_items(()).codegen_units { // TODO: Parallelise this
            for (mono_item, _,) in codegen_unit.items() {
                match (mono_item) {
                    MonoItem::Fn(instance) => {
                        let body  = tcx.instance_mir(instance.def);
                        let attrs = tcx.codegen_fn_attrs(instance.def.def_id());
                        println!();
                        println!("FUNCTION: {:#?}", instance.def.def_id());
                        // println!("{:#?}", attrs);
                        // for (bbi, bb,) in body.basic_blocks.iter().enumerate() {
                        //     println!("bb{:?}:", bbi);
                        //     for stmt in &bb.statements {
                        //         println!("  {:?}", stmt);
                        //     }
                        //     println!("  {:?}", bb.terminator().kind);
                        // }
                        // TODO
                    },
                    MonoItem::Static(def_id) => {
                        // let (is_mut, ident, ty, _,) = tcx.hir_expect_item(def_id.expect_local()).expect_static();
                        let alloc = tcx.eval_static_initializer(def_id).unwrap();
                        println!();
                        println!("STATIC {:?} = {:#?}", def_id, alloc);
                        // TODO
                    },
                    MonoItem::GlobalAsm(_) => { diag::globalasm_unsupported(tcx.dcx(), mono_item.local_span(tcx).unwrap_or(DUMMY_SP)); },
                }
            }
        }

        // for item_id in tcx.hir_crate_items(()).free_items() {
        //     let item = tcx.hir_item(item_id);
        //     match (item.kind) {
        //         ItemKind::ExternCrate(_, _,) => { },
        //         ItemKind::Use(_, _,) => { },
        //         ItemKind::Static(_, _, _, _,) => {
        //             // TODO: Statics
        //         },
        //         ItemKind::Const(_, _, _, _,) => { },
        //         ItemKind::Fn { body, .. } => {
        //             let def_id   = item_id.owner_id.to_def_id();
        //             let generics = tcx.generics_of(def_id);
        //             if (generics.requires_monomorphization(tcx)) {
        //             }
        //             // if (! generics.own_params.is_empty()) {
        //             //     // TODO: Generics
        //             //     continue;
        //             // }
        //             let instance = Instance::mono(tcx, def_id);
        //             let hir      = tcx.hir_body(body);
        //             let mir      = tcx.optimized_mir(def_id);

        //             // println!();
        //             // for (bbi, bb,) in mir.basic_blocks.iter().enumerate() {
        //             //     println!("bb{:?}:", bbi);
        //             //     for stmt in &bb.statements {
        //             //         println!("  {:?}", stmt);
        //             //     }
        //             //     println!("  {:?}", bb.terminator().kind);
        //             // }

        //             println!();
        //             // println!("{:?}", lower1::mangle_name(tcx, def_id));
        //             let cfr_tree = cfr::find_cfr_tree(&mir.basic_blocks);
        //             println!("{:#}", cfr_tree);
        //             // lower1::mir_to_dfmir(tcx, instance, mir);
        //         },
        //         ItemKind::Macro(_, _, _,) => { },
        //         ItemKind::Mod(_, _,) => { },
        //         ItemKind::ForeignMod { .. } => { },
        //         ItemKind::GlobalAsm { .. } => {
        //             diag::globalasm_unsupported(tcx.dcx(), item.span);
        //         },
        //         ItemKind::TyAlias(_, _, _,) => { },
        //         ItemKind::Enum(_, _, _,) => { },
        //         ItemKind::Struct(_, _, _,) => { },
        //         ItemKind::Union(_, _, _,) => { },
        //         ItemKind::Trait(_, _, _, _, _, _, _,) => { },
        //         ItemKind::TraitAlias(_, _, _, _,) => { },
        //         ItemKind::Impl(_,) => {
        //             // TODO: Impl
        //         }
        //     }
        // }

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
        let file_path = outputs.with_extension("dfrs-cg");
        println!("{:?}\n", file_path);
        File::create(&file_path).unwrap();
        // TODO: Write data used by the linker.

        (CodegenResults {
            modules                   : vec![
                // CompiledModule {
                //     name                  : ongoing_codegen.crate_info.local_crate_name.to_string(),
                //     kind                  : ModuleKind::Regular,
                //     object                : Some(file_path),
                //     dwarf_object          : None,
                //     bytecode              : None,
                //     assembly              : None,
                //     llvm_ir               : None,
                //     links_from_incr_cache : Vec::new()
                // }
            ],
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
