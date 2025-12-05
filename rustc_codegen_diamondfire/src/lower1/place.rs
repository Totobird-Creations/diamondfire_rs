use crate::dfmir::{
    DfMirFn,
    DfMirPlace,
    DfMirBasicVal,
    DfMirStmt
};
use rustc_middle::{
    mir::{
        Place,
        PlaceElem
    },
    ty::TyCtxt
};


pub fn place_to_dfmir<'tcx>(
    dest       : &DfMirFn,
    _tcx       : TyCtxt<'tcx>,
    place      : &Place<'tcx>,
    resolve_ty : bool
) -> DfMirPlace {
    for elem in place.projection { match (elem) {

        PlaceElem::Deref => {
            todo!()
        },

        PlaceElem::Field(_, _) => {
            todo!()
        },

        PlaceElem::Index(_) => {
            todo!()
        },

        PlaceElem::ConstantIndex { .. } => {
            todo!()
        },

        PlaceElem::Subslice { .. } => {
            todo!()
        },

        PlaceElem::Downcast(_, _) => {
            todo!()
        },

        PlaceElem::OpaqueCast(_) => {
            todo!()
        },

        PlaceElem::UnwrapUnsafeBinder(_) => {
            todo!()
        }

    } }

    let local = place.local.index();
    DfMirPlace {
        local,
        project : Vec::new(), // TODO
        ty      : resolve_ty.then(|| dest.get_local_ty(local).clone())
    }
}


pub fn place_read_dfmir(
    dest  : &mut DfMirFn,
    _tcx  : TyCtxt<'_>,
    place : DfMirPlace
) -> DfMirBasicVal {
    let mut temporary = dest.add_temporary(dest.get_local_ty(place.local).clone());
    dest.push_stmt(DfMirStmt::SetTemporary {
        temporary,
        value     : DfMirBasicVal::Local(place.local)
    });

    for elem in &place.project {
        todo!();
    }

    DfMirBasicVal::Temporary(temporary)
}
