#![feature(
    rustc_private
)]

extern crate anstream;
extern crate rustc_driver;
extern crate rustc_errors;

mod cli;
use cli::Cli;

mod ctx;
use ctx::LinkingCtx;


use bridgecg_diamondfire::{
    extern_names::ExternNameMap,
    bridge_items::BridgeItems,
    dfmir::{
        DfMirStmt,
        DfMirCall
    }
};
use std::{
    fs::File,
    io::stderr
};
use rustc_driver::default_translator;
use rustc_errors::{
    DiagCtxt,
    Diag,
    ErrorGuaranteed,
    Level,
    emitter::{
        HumanEmitter,
        Destination
    }
};


fn main() {
    // Parse command line inputs.
    let cli = Cli::parse();

    // Set up diagnostics context.
    let dcx = DiagCtxt::new(Box::new(HumanEmitter::new(
        Destination::new(Box::new(stderr()), cli.colour),
        default_translator()
    )));
    let dcx = dcx.handle();

    // Read input files.
    let mut bridge_items = BridgeItems::default();
    for input_path in cli.input_paths {
        let mut f = File::open(input_path).unwrap();
        bridge_items.append(&mut BridgeItems::read_decode(&mut f).unwrap());
    }
    let Some(extern_names) = &bridge_items.extern_names else {
        Diag::<ErrorGuaranteed>::new(dcx, Level::Error, "missing actiondump declaration")
            .with_help("you might need to import `diamondfire` or `diamondfire_sys`")
            .emit();
        dcx.abort_if_errors();
        unreachable!()
    };
    let extern_names = ExternNameMap::decode(extern_names);

    let mut ctx = LinkingCtx::default();
    // Queue all exported functions for linking.
    for (&fn_id, bridge_fn,) in &bridge_items.funcs {
        if (bridge_fn.exported) {
            ctx.queue_link_fn(fn_id);
        }
    }

    while let Some(fn_id) = ctx.pop_queued_fn() {
        let bridge_fn = bridge_items.funcs.get(&fn_id).unwrap_or_else(|| panic!("{}", fn_id));
        println!("{} ({}):", bridge_fn.name, fn_id);
        for stmt in &bridge_fn.body {
            println!("  {:?}", stmt);
            // TODO
            match (stmt) {

                DfMirStmt::CopyTL { .. } => {
                    // TODO
                },
                DfMirStmt::CopyLT { .. } => {
                    // TODO
                },
                DfMirStmt::CopyTT { .. } => {
                    // TODO
                },

                DfMirStmt::TNumber { .. } => {
                    // TODO
                },
                DfMirStmt::TStruct { .. } => {
                    // TODO
                },
                DfMirStmt::TEnum { .. } => {
                    // TODO
                },

                DfMirStmt::RawFieldGet { .. } => {
                    // TODO
                },

                DfMirStmt::Call { call, .. } => { match (call) {
                    DfMirCall::Defined(fn_id) => {
                        ctx.queue_link_fn(*fn_id);
                        // TODO
                    },
                    DfMirCall::Ptr(_) => {
                        // TODO
                    },
                    DfMirCall::Extern(_) => {
                        // TODO
                    },
                    DfMirCall::Intrinsic(_) => {
                        // TOOD
                    }
                } },
                DfMirStmt::DropCall { fn_id, .. } => {
                    ctx.queue_link_fn(*fn_id);
                    // TODO
                },

                DfMirStmt::Return => {
                    // TODO
                },

                DfMirStmt::Todo(_) => { },

            }
        }
    }

    todo!();

    dcx.abort_if_errors();
}
