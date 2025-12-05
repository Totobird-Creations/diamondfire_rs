use rustc_middle::{
    mir::Rvalue,
    ty::TyCtxt
};


pub fn rvalue_to_dfmir<'tcx>(
    _tcx   : TyCtxt<'tcx>,
    rvalue : &Rvalue<'tcx>
) {
    match (rvalue) {

        Rvalue::Use(_) => {
            todo!()
        },

        Rvalue::Repeat(_, _) => {
            todo!()
        },

        Rvalue::Ref(_, _, _) => {
            todo!()
        },

        Rvalue::ThreadLocalRef(_) => {
            todo!()
        },

        Rvalue::RawPtr(_, _) => {
            todo!()
        },

        Rvalue::Cast(_, _, _) => {
            todo!()
        },

        Rvalue::BinaryOp(_, _) => {
            todo!()
        },

        Rvalue::NullaryOp(_, _) => {
            todo!()
        },

        Rvalue::UnaryOp(_, _) => {
            todo!()
        },

        Rvalue::Discriminant(_) => {
            todo!()
        },

        Rvalue::Aggregate(_, _) => {
            todo!()
        },

        Rvalue::ShallowInitBox(_, _) => {
            todo!()
        },

        Rvalue::CopyForDeref(_) => {
            todo!()
        },

        Rvalue::WrapUnsafeBinder(_, _) => {
            todo!()
        }

    }
}
