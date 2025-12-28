use crate::ctx::LinkingCtx;
use bridgecg_diamondfire::{
    extern_names::{
        ExternName,
        ActionBlockKind
    },
    dflir::{
        DfLirBlock,
        DfLirValue
    },
    VarScope,
    Temporary
};
use static_intern::Intern as _;


pub fn extern_call_to_dflir(ctx : &mut LinkingCtx, dst : Temporary, extern_name : &str, args : &[Temporary], out : &mut Vec<DfLirBlock<'static>>) {
    let extern_entry = ctx.lookup_extern(extern_name);
    match (extern_entry) {

        ExternName::OpaqueTy => todo!(),

        ExternName::ValueTy(value_ty) => todo!(),

        ExternName::NewVar(var_scope) => todo!(),

        ExternName::ConstValue(value_ty) => todo!(),

        ExternName::Action { codeblock, action, tag_defaults } => { match (codeblock) {

            ActionBlockKind::PlayerAction => todo!(),

            ActionBlockKind::NonPlayerAction => todo!(),

            ActionBlockKind::SetVar => todo!(),

            ActionBlockKind::GameAction => todo!(),

            ActionBlockKind::SelectEntity => todo!(),

            ActionBlockKind::Control => {
                out.push(DfLirBlock::Control {
                    action : action.intern(),
                    args   : args.iter().map(|arg| {
                        DfLirValue::var_temporary(*arg, false) // TODO: Locked
                    }).collect::<Vec<_>>()
                    // TODO: Tags
                });
            }

        } },

        ExternName::Gamevalue { gamevalue } => todo!()

    }
}
