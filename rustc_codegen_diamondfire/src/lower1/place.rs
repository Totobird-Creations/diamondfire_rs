use rustc_middle::{
    mir::{
        Place,
        PlaceElem
    },
    ty::TyCtxt
};


pub fn place_read_to_dfmir<'tcx>(
    _tcx  : TyCtxt<'tcx>,
    place : &Place<'tcx>
) {
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
}
