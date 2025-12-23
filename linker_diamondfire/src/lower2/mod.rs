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
        DfLirBlock
    }
};
use static_intern::Intern as _;

mod intrinsic;
use intrinsic::intrinsic_call_to_dflir;


pub fn dfmir_to_dflir(ctx : &mut LinkingCtx, fn_id : u128, bridge_fn : &FuncItem) -> DfLirLine<'static> {
    let     head   = fn_line_head(bridge_fn);
    let mut blocks = Vec::new();

    for stmt in &bridge_fn.body {
        match (stmt) {

            DfMirStmt::CopyTL { dst, src } => {
                blocks.push(DfLirBlock::SetVar {
                    action : "="
                    // TODO: Params
                });
            },
            DfMirStmt::CopyLT { dst, src } => {
                blocks.push(DfLirBlock::SetVar {
                    action : "="
                    // TODO: Params
                });
            },
            DfMirStmt::CopyTT { dst, src } => {
                blocks.push(DfLirBlock::SetVar {
                    action : "="
                    // TODO: Params
                });
            },

            DfMirStmt::TNumber { dst, repr_value } => {
                blocks.push(DfLirBlock::SetVar {
                    action : "="
                    // TODO: Params
                });
            },
            DfMirStmt::TStruct { dst, fields } => {
                blocks.push(DfLirBlock::SetVar {
                    action : "CreateList"
                    // TODO: Params
                });
            },
            DfMirStmt::TEnum { .. } => {
                todo!()
            },

            DfMirStmt::CheckedArithBinOp { dst, op, left, right } => {
                todo!()
            },
            DfMirStmt::BoolBinOp { dst, op, left, right } => {
                todo!()
            },
            DfMirStmt::CondBinOp { dst, op, left, right } => {
                todo!()
            },

            DfMirStmt::RawFieldGet { .. } => {
                todo!()
            },

            DfMirStmt::Call { dst, call, args } => { match (call) {
                DfMirCall::Defined(fn_id) => {
                    ctx.queue_link_fn(*fn_id);
                    todo!()
                },
                DfMirCall::Ptr(_) => {
                    todo!()
                },
                DfMirCall::Extern(extern_name) => {
                    todo!("{:?}", extern_name)
                },
                DfMirCall::Intrinsic(intrinsic) => {
                    intrinsic_call_to_dflir(ctx, *dst, *intrinsic, args, &mut blocks)
                }
            } },
            DfMirStmt::DropCall { fn_id, .. } => {
                ctx.queue_link_fn(*fn_id);
                todo!()
            },

            DfMirStmt::Return => {
                todo!()
            },

            DfMirStmt::Todo(_) => { }, // TODO: Remove

        }
    }

    DfLirLine {
        head,
        blocks
    }
}


fn fn_line_head(bridge_fn : &FuncItem) -> DfLirLineHead<'static> {
    DfLirLineHead::Function { // TODO: Event
        name   : bridge_fn.name.intern(),
        hidden : ! bridge_fn.exported
    }
}
