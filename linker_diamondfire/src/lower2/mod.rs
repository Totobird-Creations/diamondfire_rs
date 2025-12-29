use crate::ctx::LinkingCtx;
use bridgecg_diamondfire::{
    bridge_items::FuncItem,
    dfmir::{
        DfMirStmt,
        DfMirCall
    },
    dflir::{
        DfLirLine,
        DfLirLineHead,
        DfLirBlock,
        DfLirValue
    },
    VarScope
};
use static_intern::Intern as _;

mod intrinsic;
use intrinsic::intrinsic_call_to_dflir;

mod externs;
use externs::extern_call_to_dflir;


pub fn dfmir_to_dflir(ctx : &mut LinkingCtx, fn_id : u128, bridge_fn : &FuncItem) -> DfLirLine<'static> {
    todo!()
    // let     head   = fn_line_head(bridge_fn);
    // let mut blocks = Vec::new();

    // for stmt in &bridge_fn.body {
    //     match (stmt) {

    //         DfMirStmt::CopyTL { dst, src } => { blocks.push(DfLirBlock::copy(DfLirValue::var_local(*dst, true), DfLirValue::var_temporary(*src, false))); },
    //         DfMirStmt::CopyLT { dst, src } => { blocks.push(DfLirBlock::copy(DfLirValue::var_temporary(*dst, true), DfLirValue::var_local(*src, false))); },
    //         DfMirStmt::CopyTT { dst, src } => { blocks.push(DfLirBlock::copy(DfLirValue::var_temporary(*dst, true), DfLirValue::var_temporary(*src, false))); },

    //         DfMirStmt::TNumber { dst, repr_value } => { blocks.push(DfLirBlock::copy(DfLirValue::var_temporary(*dst, true), DfLirValue::Number(*repr_value))); },
    //         DfMirStmt::TString { dst, value } => { blocks.push(DfLirBlock::copy(DfLirValue::var_temporary(*dst, true), DfLirValue::String(value))); },
    //         DfMirStmt::TStruct { dst, fields } => {
    //             blocks.push(DfLirBlock::SetVar {
    //                 action : "CreateList",
    //                 args   : [DfLirValue::var_temporary(*dst, true)].into_iter()
    //                     .chain(fields.iter().map(|field| DfLirValue::var_temporary(*field, false)))
    //                     .collect::<Vec<_>>()
    //             });
    //         },
    //         DfMirStmt::TEnum { .. } => {
    //             todo!()
    //         },

    //         DfMirStmt::CheckedArithBinOp { dst, op, left, right } => {
    //             todo!()
    //         },
    //         DfMirStmt::BoolBinOp { dst, op, left, right } => {
    //             todo!()
    //         },
    //         DfMirStmt::CondBinOp { dst, op, left, right } => {
    //             todo!()
    //         },

    //         DfMirStmt::RawFieldGet { .. } => {
    //             todo!()
    //         },

    //         DfMirStmt::Call { dst, call, args } => { match (call) {
    //             DfMirCall::Defined(fn_id) => {
    //                 ctx.queue_link_fn(*fn_id);
    //                 blocks.push(DfLirBlock::CallFuncion {
    //                     fn_id : *fn_id,
    //                     args  : args.iter().map(|arg| DfLirValue::Var {
    //                         scope  : VarScope::Local,
    //                         name   : format!("dfrs.{:?}", arg).intern(),
    //                         locked : false // TODO: Locked var
    //                     }).collect::<Vec<_>>()
    //                 });
    //             },
    //             DfMirCall::Ptr(_) => {
    //                 todo!()
    //             },
    //             DfMirCall::Extern(extern_name) => {
    //                 extern_call_to_dflir(ctx, *dst, extern_name, args, &mut blocks);
    //             },
    //             DfMirCall::Intrinsic(intrinsic) => {
    //                 intrinsic_call_to_dflir(ctx, *dst, *intrinsic, args, &mut blocks);
    //             }
    //         } },
    //         DfMirStmt::DropCall { fn_id, .. } => {
    //             ctx.queue_link_fn(*fn_id);
    //             todo!()
    //         },

    //         DfMirStmt::Return => {
    //             blocks.push(DfLirBlock::Control {
    //                 action : "Return",
    //                 args   : Vec::new()
    //             });
    //         },

    //         DfMirStmt::Todo(_) => { }, // TODO: Remove

    //     }
    // }

    // DfLirLine {
    //     head,
    //     blocks
    // }
}


fn fn_line_head(bridge_fn : &FuncItem) -> DfLirLineHead<'static> {
    DfLirLineHead::Function { // TODO: Event
        name   : bridge_fn.name.intern(),
        hidden : ! bridge_fn.exported
    }
}
