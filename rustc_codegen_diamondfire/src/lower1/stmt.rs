use crate::dfmir::{
    DfMirFn,
    DfMirStmt
};
use super::{
    place_to_dfmir,
    rvalue_to_dfmir
};
use rustc_middle::{
    mir::{
        Statement,
        StatementKind,
        NonDivergingIntrinsic
    },
    ty::TyCtxt
};


pub fn stmt_to_dfmir<'tcx>(
    dest : &mut DfMirFn,
    tcx  : TyCtxt<'tcx>,
    stmt : &Statement<'tcx>
) {
    match (&stmt.kind) {

        StatementKind::Assign(assign) => {
            let (place, value,) = &**assign;
            let df_place = place_to_dfmir(dest, tcx, place, false);
            let df_value = rvalue_to_dfmir(dest, tcx, value);
            dest.push_stmt(DfMirStmt::SetPlace { place : df_place, value : df_value });
        },

        StatementKind::FakeRead(_) => { unreachable!("disallowed after drop elaboration") },

        StatementKind::SetDiscriminant { .. } => {
            todo!()
        },

        StatementKind::StorageLive(_) => { },

        StatementKind::StorageDead(_) => { },

        StatementKind::Retag(_, _) => { unimplemented!() },

        StatementKind::PlaceMention(_) => {
            todo!()
        },

        StatementKind::AscribeUserType(_, _) => { unreachable!("disallowed after drop elaboration") },

        StatementKind::Coverage(_) => { unimplemented!() },

        StatementKind::Intrinsic(intrinsic) => { match (&**intrinsic) {

            NonDivergingIntrinsic::Assume(_) => {
                todo!()
            },

            NonDivergingIntrinsic::CopyNonOverlapping(_) => {
                todo!()
            }

        } },

        StatementKind::ConstEvalCounter => { },

        StatementKind::Nop => { },

        StatementKind::BackwardIncompatibleDropHint { .. } => { }

    }
}
