use super::Lower1Ctx;
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirLocal,
    DfMirTemporary
};
use rustc_middle::mir::{
    Place,
    ProjectionElem
};


pub fn place_load_to_dfmir<'tcx>(
    ctx   : &mut Lower1Ctx<'tcx, '_>,
    place : &Place<'tcx>,
    out   : &mut Vec<DfMirStmt>
) -> DfMirTemporary {

    let mut temp = ctx.next_temp();
    out.push(DfMirStmt::CopyLT {
        src : DfMirLocal(place.local.as_usize()),
        dst : temp
    });

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

    temp
}
