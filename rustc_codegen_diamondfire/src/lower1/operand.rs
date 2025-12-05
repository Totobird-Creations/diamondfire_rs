use crate::dfmir::{
    DfMirFn,
    DfMirBasicVal
};
use super::{
    place_to_dfmir,
    place_read_dfmir
};
use rustc_middle::{
    mir::{
        Operand,
        Const,
        ConstValue,
        interpret::Scalar
    },
    ty::{
        TyCtxt,
        TyKind
    }
};


pub fn operand_to_dfmir<'tcx>(
    dest    : &mut DfMirFn,
    tcx     : TyCtxt<'tcx>,
    operand : &Operand<'tcx>
) -> DfMirBasicVal {
    match (operand) {

        Operand::Copy(place) | Operand::Move(place) => {
            let df_place = place_to_dfmir(dest, tcx, place, true);
            place_read_dfmir(dest, tcx, df_place)
        },

        Operand::Constant(operand) => { const_to_dfmir(dest, tcx, &operand.const_) }

    }
}


pub fn const_to_dfmir<'tcx>(
    _dest  : &mut DfMirFn,
    _tcx   : TyCtxt<'tcx>,
    const_ : &Const<'tcx>
) -> DfMirBasicVal {
    match (const_) {

        Const::Ty(_, _) => { todo!() },

        Const::Unevaluated(_, _) => { todo!() },

        Const::Val(value, ty) => { match (value) {

            ConstValue::Scalar(scalar) => { match (scalar) {

                Scalar::Int(value) => { match (ty.kind()) {

                    TyKind::Int(_) => {
                        let size = value.size();
                        DfMirBasicVal::Int {
                            value : value.to_int(size),
                            size  : size.bits()
                        }
                    },

                    TyKind::Uint(_) => {
                        let size = value.size();
                        DfMirBasicVal::UInt {
                            value : value.to_uint(size),
                            size  : size.bits()
                        }
                    },

                    _ => todo!("{}", ty)

                } },

                Scalar::Ptr(_, _) => { todo!() }

            } },

            ConstValue::ZeroSized => { todo!() },

            ConstValue::Slice { .. } => { todo!() },

            ConstValue::Indirect { .. } => { todo!() }

        } }

    }
}
