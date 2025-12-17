use crate::diag;
use super::{
    Lower1Ctx,
    place_store_to_dfmir,
    rvalue_to_dfmir
};
use bridgecg_diamondfire::dfmir::DfMirStmt;
use rustc_middle::mir::{
    Statement,
    StatementKind
};


pub fn stmt_to_dfmir<'tcx>(
    ctx  : &mut Lower1Ctx<'tcx, '_>,
    stmt : &Statement<'tcx>,
    out  : &mut Vec<DfMirStmt>
) {
    match (&stmt.kind) {

        StatementKind::Assign(box (place, rvalue,))
        => {
            let rvalue_df = rvalue_to_dfmir(ctx, rvalue, out);
            place_store_to_dfmir(ctx, place, rvalue_df, out);
        },

        StatementKind::SetDiscriminant { .. }
        => todo!(),

        StatementKind::Intrinsic(_)
        => todo!(),

        StatementKind::FakeRead(_)
        | StatementKind::AscribeUserType(_, _)
        => { diag::disallowed_post_drop_elaboration(); },

        StatementKind::StorageLive(_)
        | StatementKind::StorageDead(_)
        | StatementKind::Retag(_, _)
        | StatementKind::PlaceMention(_)
        | StatementKind::Coverage(_)
        | StatementKind::ConstEvalCounter
        | StatementKind::Nop
        | StatementKind::BackwardIncompatibleDropHint { .. }
        => { }

    }
}
