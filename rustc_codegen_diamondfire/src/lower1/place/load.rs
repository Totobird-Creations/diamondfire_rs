use super::Lower1Ctx;
use bridgecg_diamondfire::{
    dfmir::DfMirStmt,
    Local,
    Temporary
};
use rustc_middle::{
    mir::{
        Place,
        ProjectionElem
    },
    ty::Ty
};


pub fn place_load_to_dfmir<'tcx>(
    ctx   : &mut Lower1Ctx<'tcx, '_>,
    place : &Place<'tcx>,
    out   : &mut Vec<DfMirStmt>
) -> (Temporary, Ty<'tcx>,) {

    let mut temp = ctx.next_temp();
    let mut ty   = ctx.body.local_decls.get(place.local).unwrap().ty;
    out.push(DfMirStmt::CopyLT {
        src : Local(place.local.as_usize()),
        dst : temp
    });

    for elem in place.projection { match (elem) {
        ProjectionElem::Deref => {
            out.push(DfMirStmt::todo(&format!("deref ptr {:?}", temp)));
            ty = ty.builtin_deref(true).unwrap();
        },
        ProjectionElem::Field(field_idx, field_ty) => {
            out.push(DfMirStmt::todo(&format!("read field {:?}.{}", temp, field_idx.as_usize())));
            ty = field_ty;
        },
        ProjectionElem::Index(_) => todo!(),
        ProjectionElem::ConstantIndex { .. } => todo!(),
        ProjectionElem::Subslice { .. } => todo!(),
        ProjectionElem::Downcast(_, _) => todo!(),
        ProjectionElem::OpaqueCast(_) => todo!(),
        ProjectionElem::UnwrapUnsafeBinder(_) => todo!(),
    } }

    (temp, ty,)
}
