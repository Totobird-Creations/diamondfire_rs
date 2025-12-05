use crate::dfmir::{
    DfMirFn,
    DfMirBasicVal,
    DfMirStmt,
    DfMirTy,
    DfMirNumTy
};
use rustc_middle::{
    mir::BinOp,
    ty::TyCtxt
};


pub fn binop_to_dfmir(
    dest  : &mut DfMirFn,
    _tcx  : TyCtxt<'_>,
    op    : BinOp,
    left  : DfMirBasicVal,
    right : DfMirBasicVal
) -> DfMirBasicVal {
    match (op) {

        BinOp::Add => {
            let left_ty  = left.ty(dest);
            let right_ty = right.ty(dest);
            assert_eq!(left_ty, right_ty);
            let temporary = dest.add_temporary(left_ty.into_owned());
            dest.push_stmt(DfMirStmt::UncheckedAdd {
                temporary, left, right
            });
            DfMirBasicVal::Temporary(temporary)
        },

        BinOp::AddUnchecked => todo!(),

        BinOp::AddWithOverflow => todo!(),

        BinOp::Sub => {
            let left_ty  = left.ty(dest);
            let right_ty = right.ty(dest);
            assert_eq!(left_ty, right_ty);
            let temporary = dest.add_temporary(left_ty.into_owned());
            dest.push_stmt(DfMirStmt::UncheckedSub {
                temporary, left, right
            });
            DfMirBasicVal::Temporary(temporary)
        },

        BinOp::SubUnchecked => todo!(),

        BinOp::SubWithOverflow => todo!(),

        BinOp::Mul => todo!(),

        BinOp::MulUnchecked => todo!(),

        BinOp::MulWithOverflow => todo!(),

        BinOp::Div => todo!(),

        BinOp::Rem => todo!(),

        BinOp::BitXor => todo!(),

        BinOp::BitAnd => todo!(),

        BinOp::BitOr => todo!(),

        BinOp::Shl => todo!(),

        BinOp::ShlUnchecked => todo!(),

        BinOp::Shr => todo!(),

        BinOp::ShrUnchecked => todo!(),

        BinOp::Eq => todo!(),

        BinOp::Lt => todo!(),

        BinOp::Le => todo!(),

        BinOp::Ne => todo!(),

        BinOp::Ge => todo!(),

        BinOp::Gt => {
            let left_ty  = left.ty(dest);
            let right_ty = right.ty(dest);
            assert_eq!(left_ty, right_ty);
            let temporary = dest.add_temporary(DfMirTy::Num(DfMirNumTy::Bool));
            dest.push_stmt(DfMirStmt::GreaterThan {
                temporary, left, right
            });
            DfMirBasicVal::Temporary(temporary)
        },

        BinOp::Cmp => todo!(),

        BinOp::Offset => todo!()

    }
}
