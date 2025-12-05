use rustc_middle::{
    mir::Operand,
    ty::TyCtxt
};


pub fn operand_to_dfmir<'tcx>(
    _tcx    : TyCtxt<'tcx>,
    operand : &Operand<'tcx>
) {
    match (operand) {

        Operand::Copy(_) => { todo!() },

        Operand::Move(_) => { todo!() },

        Operand::Constant(_) => { todo!() }

    }
}
