use super::{
    Lower1Ctx,
    place_load_to_dfmir
};
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirTemporary
};
use rustc_middle::mir::{
    Operand,
    Const,
    ConstValue,
    interpret::{
        Scalar,
        GlobalAlloc
    }
};

mod scalar_int;
pub use scalar_int::scalar_int_to_dfmir;

mod alloc;
pub use alloc::alloc_to_dfmir;


pub fn operand_to_dfmir<'tcx>(
    ctx     : &mut Lower1Ctx<'tcx, '_>,
    operand : &Operand<'tcx>,
    out     : &mut Vec<DfMirStmt>
) -> DfMirTemporary {
    match (operand) {

        Operand::Copy(place)
        | Operand::Move(place)
        => { place_load_to_dfmir(ctx, place, out) },

        Operand::Constant(box const_operand) => { match (const_operand.const_) {

            Const::Ty(_, _) => todo!(),

            Const::Unevaluated(_, _) => todo!(),

            Const::Val(val, ty) => { match (val) {

                ConstValue::Scalar(scalar) => { match (scalar) {

                    Scalar::Int(scalar_int) => { scalar_int_to_dfmir(ctx, ty, scalar_int, out) },

                    Scalar::Ptr(pointer, _) => todo!(),

                } },

                ConstValue::ZeroSized => todo!(),

                ConstValue::Slice { alloc_id, meta } => todo!(),

                ConstValue::Indirect { alloc_id, offset } => { match (ctx.tcx.global_alloc(alloc_id)) {

                    GlobalAlloc::Function { .. } => todo!(),

                    GlobalAlloc::VTable(_, _) => todo!(),

                    GlobalAlloc::Static(_) => todo!(),

                    GlobalAlloc::Memory(const_allocation) => { alloc_to_dfmir(ctx, ty, const_allocation.inner(), offset, out, const_operand.span) },

                    GlobalAlloc::TypeId { .. } => todo!()

                } }

            } }

        } }

    }
}
