use super::{
    Lower1Ctx,
    place_load_to_dfmir,
    operand_to_dfmir
};
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirTemporary
};
use rustc_middle::mir::{
    Rvalue,
    CastKind,
    AggregateKind
};


pub fn rvalue_to_dfmir<'tcx>(
    ctx    : &mut Lower1Ctx<'tcx, '_>,
    rvalue : &Rvalue<'tcx>,
    out    : &mut Vec<DfMirStmt>
) -> DfMirTemporary {
    match (rvalue) {

        Rvalue::Use(operand) => { operand_to_dfmir(ctx, operand, out) },

        Rvalue::Repeat(operand, _) => todo!(),

        Rvalue::Ref(region, borrow_kind, place) => {
            let place_df = place_load_to_dfmir(ctx, place, out).0;
            out.push(DfMirStmt::todo(&format!("ref val {:?}", place_df)));
            place_df
        },

        Rvalue::ThreadLocalRef(def_id) => todo!(),

        Rvalue::RawPtr(raw_ptr_kind, place) => todo!(),

        Rvalue::Cast(kind, operand, ty) => { match (kind) {

            CastKind::PointerExposeProvenance => todo!(),

            CastKind::PointerWithExposedProvenance => todo!(),

            CastKind::PointerCoercion(pointer_coercion, coercion_source) => todo!(),

            CastKind::IntToInt => todo!(),

            CastKind::FloatToInt => todo!(),

            CastKind::FloatToFloat => todo!(),

            CastKind::IntToFloat => todo!(),

            CastKind::PtrToPtr => todo!(),

            CastKind::FnPtrToPtr => todo!(),

            CastKind::Transmute => todo!(),

            CastKind::Subtype => todo!()

        } },

        Rvalue::BinaryOp(bin_op, _) => todo!(),

        Rvalue::NullaryOp(null_op) => todo!(),

        Rvalue::UnaryOp(un_op, operand) => todo!(),

        Rvalue::Discriminant(place) => todo!(),

        Rvalue::Aggregate(box kind, fields) => { match (kind) {

            AggregateKind::Array(ty) => todo!(),

            AggregateKind::Tuple => todo!(),

            AggregateKind::Adt(def_id, variant_idx, generics, user_type_annotation_index, field_idx) => {
                let mut field_temps = Vec::new();
                let adt_def = ctx.tcx.adt_def(def_id);
                let variant = adt_def.variant(*variant_idx);
                for (field_idx, field_def,) in variant.fields.iter_enumerated() {
                    // let field_ty = field_def.ty(ctx.tcx, generics);
                    let field_df = operand_to_dfmir(ctx, fields.get(field_idx).unwrap(), out);
                    field_temps.push(field_df);
                }
                let dst = ctx.next_temp();
                out.push(DfMirStmt::TStruct { dst, fields : field_temps });
                dst
            },

            AggregateKind::Closure(def_id, raw_list) => todo!(),

            AggregateKind::Coroutine(def_id, raw_list) => todo!(),

            AggregateKind::CoroutineClosure(def_id, raw_list) => todo!(),

            AggregateKind::RawPtr(ty, mutability) => todo!()

        } },

        Rvalue::ShallowInitBox(operand, ty) => todo!(),

        Rvalue::CopyForDeref(place) => todo!(),

        Rvalue::WrapUnsafeBinder(operand, ty) => todo!()

    }
}
