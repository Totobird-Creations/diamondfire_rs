use super::Lower1Ctx;
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirTemporary
};
use rustc_middle::ty::{
    Ty,
    TyKind,
    IntTy,
    UintTy,
    ScalarInt
};


pub fn scalar_int_to_dfmir<'tcx>(
    ctx        : &mut Lower1Ctx<'tcx, '_>,
    ty         : Ty<'tcx>,
    scalar_int : ScalarInt,
    out        : &mut Vec<DfMirStmt>
) -> DfMirTemporary {
    match (ty.kind()) {

        TyKind::Int(int_ty) => {
            let df_value = { match (int_ty) {
                IntTy::I8                 => { (scalar_int.to_i8() as i64) * 1000 },
                IntTy::I16                => { (scalar_int.to_i16() as i64) * 1000 },
                IntTy::I32 | IntTy::Isize => { (scalar_int.to_i32() as i64) * 1000 },
                IntTy::I64                => { scalar_int.to_i64() },
                IntTy::I128               => todo!()
            } };
            let dst = ctx.next_temp();
            out.push(DfMirStmt::TNumber { dst, df_value });
            dst
        },

        TyKind::Uint(uint_ty) => {
            let df_value = { match (uint_ty) {
                UintTy::U8                  => { (scalar_int.to_u8() as i64) * 1000 },
                UintTy::U16                 => { (scalar_int.to_u16() as i64) * 1000 },
                UintTy::U32 | UintTy::Usize => { (scalar_int.to_u32() as i64) * 1000 },
                UintTy::U64                 => todo!(),
                UintTy::U128                => todo!()
            } };
            let dst = ctx.next_temp();
            out.push(DfMirStmt::TNumber { dst, df_value });
            dst
        },

        TyKind::Bool => {
            let dst = ctx.next_temp();
            out.push(DfMirStmt::TNumber { dst, df_value : if (scalar_int.try_to_bool().unwrap()) { 1000 } else { 0 } });
            dst
        },

        TyKind::Float(_) => todo!(),

        TyKind::Char => todo!(),

        tyk => unimplemented!("{:?}", tyk)
    }
}
