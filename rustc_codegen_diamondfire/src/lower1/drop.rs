use crate::hash::HashingUtil;
use super::Lower1Ctx;
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirTemporary
};
use rustc_abi::VariantIdx;
use rustc_middle::ty::{
    Ty,
    TyKind,
    TypingEnv,
    AdtKind
};


pub fn drop_to_dfmir<'tcx>(
    ctx : &mut Lower1Ctx<'tcx, '_>,
    ty  : Ty<'tcx>,
    src : DfMirTemporary,
    out : &mut Vec<DfMirStmt>
) {

    match (ty.kind()) {

        TyKind::Bool | TyKind::Char | TyKind::Int(_) | TyKind::Uint(_) | TyKind::Float(_)
        => { },

        TyKind::Adt(adt_def, generics) => { match (adt_def.adt_kind()) {

            AdtKind::Struct => {
                // Call destructor for self type.
                if let Some(destructor) = adt_def.destructor(ctx.tcx) {
                    out.push(DfMirStmt::DropCall {
                        fn_id : HashingUtil::hash_fn_def(ctx.tcx, destructor.did, *generics),
                        value : src
                    });
                }
                // Recursively add destructor calls for fields.
                let variant = adt_def.variant(VariantIdx::from_usize(0));
                for (field_idx, field_def,) in variant.fields.iter_enumerated() {
                    let field_ty  = field_def.ty(ctx.tcx, generics);
                    let field_src = ctx.next_temp();
                    out.push(DfMirStmt::RawFieldGet { dst : field_src, src, field : field_idx.as_usize() });
                    drop_to_dfmir(ctx, field_ty, field_src, out);
                }
            },

            AdtKind::Union => todo!(),

            AdtKind::Enum => todo!()

        } },

        TyKind::Foreign(_) => todo!(),

        TyKind::Str => todo!(),

        TyKind::Array(_, _) => todo!(),

        TyKind::Pat(_, _) => todo!(),

        TyKind::Slice(_) => todo!(),

        TyKind::RawPtr(_, mutability) => todo!(),

        TyKind::Ref(_, _, mutability) => todo!(),

        TyKind::FnDef(_, _) => todo!(),

        TyKind::FnPtr(binder, fn_header) => todo!(),

        TyKind::UnsafeBinder(unsafe_binder_inner) => todo!(),

        TyKind::Dynamic(_, _) => todo!(),

        TyKind::Closure(_, _) => todo!(),

        TyKind::CoroutineClosure(_, _) => todo!(),

        TyKind::Coroutine(_, _) => todo!(),

        TyKind::CoroutineWitness(_, _) => todo!()
        ,

        TyKind::Tuple(_) => todo!(),

        TyKind::Never
        | TyKind::Alias(_, _)
        | TyKind::Param(_)
        | TyKind::Bound(_, _)
        | TyKind::Placeholder(_)
        | TyKind::Infer(_)
        | TyKind::Error(_)
        => unreachable!()

    }
}
