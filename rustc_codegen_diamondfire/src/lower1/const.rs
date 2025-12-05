use rustc_middle::{
    mir::{
        Const,
        ConstValue,
        interpret::Scalar,
    },
    ty::{
        TyCtxt,
        Const as TyConst,
        ConstKind as TyConstKind
    }
};


pub fn const_to_dfmir<'tcx>(
    _tcx    : TyCtxt<'tcx>,
    r#const : &Const<'tcx>
) {
    match (r#const) {

        Const::Ty(_, _) => { todo!() },

        Const::Unevaluated(_, _) => { todo!() },

        Const::Val(value, _) => { match (value) {

            ConstValue::Scalar(scalar) => { match (scalar) {

                Scalar::Int(_) => { todo!() },

                Scalar::Ptr(_, _) => { print!("TODOPTR") }

            } },

            ConstValue::ZeroSized => { todo!() },

            ConstValue::Slice { .. } => { todo!() },

            ConstValue::Indirect { .. } => { todo!() }

        } }

    }
}


pub fn tyconst_to_dfmir<'tcx>(
    _tcx    : TyCtxt<'tcx>,
    tyconst : &TyConst<'tcx>
) {
    match (tyconst.kind()) {

        TyConstKind::Param(_) => { todo!() },

        TyConstKind::Infer(_) => { todo!() },

        TyConstKind::Bound(_, _) => { todo!() },

        TyConstKind::Placeholder(_) => { todo!() },

        TyConstKind::Unevaluated(_) => { todo!() },

        TyConstKind::Value(_) => { todo!() },

        TyConstKind::Error(_) => { todo!() },

        TyConstKind::Expr(_) => { todo!() }

    }
}
