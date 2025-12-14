#![feature(
    rustc_private,
    f128,
    debug_closure_helpers,
    map_try_insert,
    assert_matches
)]


extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_codegen_llvm;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_query_system;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_stable_hash;

use bridgecg_diamondfire::items::{
    BridgeItems,
    FunctionItem
};
use core::any::Any;
use std::fs::File;
use rustc_ast::LitKind;
use rustc_codegen_ssa::{
    traits::CodegenBackend,
    TargetConfig,
    CodegenResults,
    CrateInfo
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{
    ItemKind,
    ConstItemRhs,
    Body,
    Expr,
    ExprKind,
    Attribute,
    attrs::{
        AttributeKind,
        Linkage
    }
};
use rustc_middle::{
    middle::codegen_fn_attrs::CodegenFnAttrFlags,
    mir::mono::MonoItem,
    ty::TyCtxt
};
use rustc_query_system::dep_graph::{
    dep_node::WorkProductId,
    WorkProduct
};
use rustc_session::{
    config::{
        OutputFilenames,
        CrateType
    },
    Session
};
use rustc_span::{
    DUMMY_SP,
    source_map::Spanned
};


pub mod cfr;
pub mod dfmir;

// pub mod lower1;

pub mod diag;

pub mod hash;
use hash::HashingUtil;


struct CrateToJoin {
    crate_info   : CrateInfo,
    bridge_items : BridgeItems
}


pub struct DiamondfireCodegen;

impl CodegenBackend for DiamondfireCodegen {

    fn name(&self) -> &'static str { "diamondfire" }

    fn locale_resource(&self) -> &'static str { "" }

    fn target_config(&self, _sess : &Session) -> TargetConfig { TargetConfig {
        target_features          : Vec::new(),
        unstable_target_features : Vec::new(),
        has_reliable_f16         : false,
        has_reliable_f16_math    : false,
        has_reliable_f128        : false,
        has_reliable_f128_math   : false
    } }


    fn codegen_crate<'tcx>(&self, tcx : TyCtxt<'tcx>) -> Box<dyn Any> {
        let     crate_info   = CrateInfo::new(tcx, "diamondfire".to_string());
        let mut bridge_items = BridgeItems::default();

        let crate_name = crate_info.local_crate_name.to_string();
        if (crate_name == "compiler_builtins" || crate_name == "core") { // TODO: Remove
            return Box::new(CrateToJoin { crate_info, bridge_items });
        }


        // Search for items which declare information required by either codegen or the linker.
        for item_id in tcx.hir_crate_items(()).definitions() {

            // `__PRIVATE_DIAMONDFIRE_SYS__EXTERN_NAMES` is a `bridgecg_diamondfire::extern_names::ExternNameMap` encoded as `bincode::config::standard()`.
            // The constant is passed to linker_diamondfire.
            if (tcx.opt_item_name(item_id).is_some_and(|name| name.as_str() == "__PRIVATE_DIAMONDFIRE_SYS__EXTERN_NAMES")) {
                assert!(bridge_items.extern_names.is_none());
                let item = tcx.hir_expect_item(item_id);
                // `__PRIVATE_DIAMONDFIRE_SYS__EXTERN_NAMES` must be a `const` with value set to a `&[u8]`.
                let ItemKind::Const(_, _, _, ConstItemRhs::Body(body_id)) = item.kind else { unreachable!(); };
                // Since `diamondfire_sys` declares it using `include_bytes!`, we assume it is a bytestr literal without bothering to evaluate it.
                let Body { params : [ ], value : Expr { kind : ExprKind::Lit(Spanned { node : LitKind::ByteStr(symbol, _), .. }), .. } } = tcx.hir_body(body_id) else { unreachable!() };
                bridge_items.extern_names = Some(symbol.as_byte_str().to_vec());
            }

        }


        for codegen_unit in tcx.collect_and_partition_mono_items(()).codegen_units { // TODO: Parallelise this
            for (mono_item, mono_item_data,) in codegen_unit.items() {
                let src_doc  = tcx.get_all_attrs(mono_item.def_id()).iter()
                    .filter_map(|attr| {
                        if let Attribute::Parsed(AttributeKind::DocComment { comment, .. }) = attr { Some(comment.as_str().trim()) }
                        else { None }
                    })
                    .flat_map(|attr_comment| attr_comment.split("\n").map(|line| line.trim()))
                    .flat_map(|attr_comment| ["\n", attr_comment,]).skip(1)
                    .collect::<String>();
                let name     = mono_item.symbol_name(tcx).to_string();
                let attrs    = tcx.codegen_fn_attrs(mono_item.def_id());
                match (mono_item) {

                    MonoItem::Fn(instance) => {
                        let src_name = tcx.value_path_str_with_args(mono_item.def_id(), instance.args);
                        let unique_id = HashingUtil::hash_fn_def(tcx, instance.def.def_id(), instance.args);
                        let body      = tcx.instance_mir(instance.def);
                        // println!("FUNCTION: {:?}{:?} {}", tcx.opt_item_name(instance.def.def_id()), instance.args, );
                        // for (bbi, bb,) in body.basic_blocks.iter().enumerate() {
                        //     println!("bb{:?}:", bbi);
                        //     for stmt in &bb.statements {
                        //         println!("  {:?}", stmt);
                        //     }
                        //     if let rustc_middle::mir::TerminatorKind::Call { func, .. } = &bb.terminator().kind {
                        //         if let rustc_middle::ty::TyKind::FnDef(def_id, genargs) = func.ty(body, tcx).kind() {
                        //             if (tcx.is_foreign_item(*def_id)) {
                        //                 let extern_fn_attrs = tcx.codegen_fn_attrs(*def_id);
                        //                 assert!(extern_fn_attrs.flags.contains(CodegenFnAttrFlags::FOREIGN_ITEM));
                        //                 println!("  {:?} {} (extern {:?})", bb.terminator().kind, HashingUtil::hash_fn_def(tcx, *def_id, *genargs), extern_fn_attrs.symbol_name.unwrap_or_else(|| tcx.item_ident(*def_id).name));
                        //             } else {
                        //                 println!("  {:?} {} (fndef)", bb.terminator().kind, HashingUtil::hash_fn_def(tcx, *def_id, *genargs));
                        //             }
                        //         } else if let rustc_middle::ty::TyKind::Closure(def_id, genargs) = func.ty(body, tcx).kind() {
                        //             println!("  {:?} {} (closure)", bb.terminator().kind, HashingUtil::hash_fn_def(tcx, *def_id, *genargs));
                        //         } else {
                        //             println!("  {:?}", bb.terminator().kind);
                        //         }
                        //     } else {
                        //         println!("  {:?}", bb.terminator().kind);
                        //     }
                        // }
                        let cfr_tree = cfr::find_cfr_tree(&body.basic_blocks);
                        // println!("{:#}", cfr_tree);
                        // TODO
                        bridge_items.functions.insert(unique_id, FunctionItem {
                            src_name,
                            src_doc,
                            name,
                            exported : (
                                (mono_item_data.linkage != Linkage::Internal)
                                || attrs.flags.contains(CodegenFnAttrFlags::NO_MANGLE)
                                || attrs.symbol_name.is_some()
                            ),
                            inline   : mono_item_data.inlined
                        });
                    },

                    MonoItem::Static(def_id) => {
                        let alloc = tcx.eval_static_initializer(def_id).unwrap();
                        // let (is_mut, ident, ty, _,) = tcx.hir_expect_item(def_id.expect_local()).expect_static();
                        // println!("STATIC {:?} = {:#?}", def_id, alloc);
                        // TODO
                    },

                    MonoItem::GlobalAsm(_) => { diag::globalasm_unsupported(tcx.dcx(), mono_item.local_span(tcx).unwrap_or(DUMMY_SP)); }

                }
            }
        }

        Box::new(CrateToJoin {
            crate_info,
            bridge_items
        })
    }


    fn join_codegen(&self,
        ongoing_codegen : Box<dyn Any>,
        _sess           : &Session,
        outputs         : &OutputFilenames
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>,) {
        let ongoing_codegen = ongoing_codegen.downcast::<CrateToJoin>().unwrap();

        let mut file_path = outputs.with_extension("dfrs-cg");
        if (! ongoing_codegen.crate_info.crate_types.contains(&CrateType::Executable)) {
            file_path.set_file_name(format!("lib{}", file_path.file_name().unwrap().to_str().unwrap()));
        }
        ongoing_codegen.bridge_items.encode_write(&mut File::create(&file_path).unwrap()).unwrap();

        (CodegenResults {
            modules          : vec![
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


}


#[unsafe(no_mangle)]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(DiamondfireCodegen)
}
