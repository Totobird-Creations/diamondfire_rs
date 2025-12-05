use rustc_middle::{
    mir::{
        Statement,
        StatementKind,
        NonDivergingIntrinsic
    },
    ty::TyCtxt
};


pub fn stmt_to_dfmir<'tcx>(
    _tcx : TyCtxt<'tcx>,
    stmt : &Statement<'tcx>
) {
    match (&stmt.kind) {

        StatementKind::Assign(_) => {
            todo!()
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
