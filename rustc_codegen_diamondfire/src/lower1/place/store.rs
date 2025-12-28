use super::Lower1Ctx;
use bridgecg_diamondfire::{
    dfmir::DfMirStmt,
    Local,
    Temporary
};
use rustc_middle::mir::{
    Place,
    ProjectionElem
};


pub fn place_store_to_dfmir<'tcx>(
    ctx   : &mut Lower1Ctx<'tcx, '_>,
    place : &Place<'tcx>,
    src   : Temporary,
    out   : &mut Vec<DfMirStmt>
) {
    let local = Local(place.local.as_usize());

    // Start by loading the place.
    // Chances are, this will be optimised out.

    let mut temp = ctx.next_temp();
    out.push(DfMirStmt::CopyLT { src : local, dst : temp });

    for elem in place.projection { match (elem) {
        ProjectionElem::Deref => todo!(),
        ProjectionElem::Field(_, _) => todo!(),
        ProjectionElem::Index(_) => todo!(),
        ProjectionElem::ConstantIndex { .. } => todo!(),
        ProjectionElem::Subslice { .. } => todo!(),
        ProjectionElem::Downcast(_, _) => todo!(),
        ProjectionElem::OpaqueCast(_) => todo!(),
        ProjectionElem::UnwrapUnsafeBinder(_) => todo!(),
    } }

    // We don't need the last item because it will just be overwritten.
    _ = out.pop(); // TODO: Make sure this still works with ProjectionElem::Deref.

    // Write back to the place.

    out.push(DfMirStmt::CopyTT { src, dst : temp });

    for elem in place.projection { match (elem) {
        ProjectionElem::Deref => todo!(),
        ProjectionElem::Field(_, _) => todo!(),
        ProjectionElem::Index(_) => todo!(),
        ProjectionElem::ConstantIndex { .. } => todo!(),
        ProjectionElem::Subslice { .. } => todo!(),
        ProjectionElem::Downcast(_, _) => todo!(),
        ProjectionElem::OpaqueCast(_) => todo!(),
        ProjectionElem::UnwrapUnsafeBinder(_) => todo!(),
    } }

    out.push(DfMirStmt::CopyTL { src : temp, dst : local });

}
