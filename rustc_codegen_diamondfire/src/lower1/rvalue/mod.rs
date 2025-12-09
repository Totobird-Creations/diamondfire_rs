use crate::dfmir::{
    DfMirFn,
    DfMirBasicVal
};
use super::operand_to_dfmir;
use rustc_middle::{
    mir::Rvalue,
    ty::TyCtxt
};


mod binop;
pub use binop::*;


pub fn rvalue_to_dfmir<'tcx>(
    dest   : &mut DfMirFn,
    tcx    : TyCtxt<'tcx>,
    rvalue : &Rvalue<'tcx>
) -> DfMirBasicVal {
    match (rvalue) {

        Rvalue::Use(operand) => { operand_to_dfmir(dest, tcx, operand) },

        Rvalue::Repeat(_, _) => {
            todo!()
        },

        Rvalue::Ref(_, _, _) => {
            todo!()
        },

        Rvalue::ThreadLocalRef(_) => {
            todo!()
        },

        Rvalue::RawPtr(_, _) => {
            todo!()
        },

        Rvalue::Cast(_, _, _) => {
            todo!()
        },

        Rvalue::BinaryOp(op, values) => {
            let (left, right,) = &**values;
            let df_left  = operand_to_dfmir(dest, tcx, left);
            let df_right = operand_to_dfmir(dest, tcx, right);
            binop_to_dfmir(dest, tcx, *op, df_left, df_right)
        },

        Rvalue::NullaryOp(_) => {
            todo!()
        },

        Rvalue::UnaryOp(_, _) => {
            todo!()
        },

        Rvalue::Discriminant(_) => {
            todo!()
        },

        Rvalue::Aggregate(_, _) => {
            todo!()
        },

        Rvalue::ShallowInitBox(_, _) => {
            todo!()
        },

        Rvalue::CopyForDeref(_) => {
            todo!()
        },

        Rvalue::WrapUnsafeBinder(_, _) => {
            todo!()
        }

    }
}
