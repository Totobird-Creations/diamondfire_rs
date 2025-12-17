use super::{
    Lower1Ctx,
    operand_to_dfmir
};
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirTemporary
};
use rustc_middle::mir::{
    Rvalue,
    CastKind
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
            println!("{:#?}", out);
            todo!()
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

        Rvalue::Aggregate(aggregate_kind, index_vec) => todo!(),

        Rvalue::ShallowInitBox(operand, ty) => todo!(),

        Rvalue::CopyForDeref(place) => todo!(),

        Rvalue::WrapUnsafeBinder(operand, ty) => todo!()

    }
}
