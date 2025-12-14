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
    items::BridgeItems
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
    for (&fn_id, bridge_fn,) in &bridge_items.functions {
        if (bridge_fn.exported) {
            ctx.queue_link_fn(fn_id);
        }
    }

    while let Some(fn_id) = ctx.pop_queued_fn() {
        let bridge_fn = bridge_items.functions.get(&fn_id).unwrap();
        println!("{}: {:#?}", fn_id, bridge_fn);
    }

    todo!();

    dcx.abort_if_errors();
}
