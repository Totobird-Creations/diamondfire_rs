use crate::dfmir::{
    DfMirTy,
    DfMirNumTy
};
use rustc_middle::ty::{
    TyCtxt,
    Ty,
    TyKind,
    IntTy,
    UintTy,
    FloatTy
};


pub fn ty_to_dfmir<'tcx>(
    _tcx : TyCtxt<'tcx>,
    ty   : &Ty<'tcx>
) -> DfMirTy {
    match (ty.kind()) {

        TyKind::Bool => todo!(),

        TyKind::Char => todo!(),

        TyKind::Int(ty) => { DfMirTy::Num(DfMirNumTy::Int { size : { match (ty) {
            IntTy::Isize => None,
            IntTy::I8    => Some(8),
            IntTy::I16   => Some(16),
            IntTy::I32   => Some(32),
            IntTy::I64   => Some(64),
            IntTy::I128  => Some(128)
        } }, unsigned : false }) },

        TyKind::Uint(ty) => { DfMirTy::Num(DfMirNumTy::Int { size : { match (ty) {
            UintTy::Usize => None,
            UintTy::U8    => Some(8),
            UintTy::U16   => Some(16),
            UintTy::U32   => Some(32),
            UintTy::U64   => Some(64),
            UintTy::U128  => Some(128)
        } }, unsigned : true }) },

        TyKind::Float(ty) => { DfMirTy::Num(DfMirNumTy::Float { size : { match (ty) {
            FloatTy::F16  => 16,
            FloatTy::F32  => 32,
            FloatTy::F64  => 64,
            FloatTy::F128 => 128
        } } }) },

        TyKind::Adt(_, _) => todo!(),

        TyKind::Foreign(_) => todo!(),

        TyKind::Str => todo!(),

        TyKind::Array(_, _) => todo!(),

        TyKind::Pat(_, _) => todo!(),

        TyKind::Slice(_) => todo!(),

        TyKind::RawPtr(_, _) => todo!(),

        TyKind::Ref(_, _, _) => todo!(),

        TyKind::FnDef(_, _) => todo!(),

        TyKind::FnPtr(_, _) => todo!(),

        TyKind::UnsafeBinder(_) => todo!(),

        TyKind::Dynamic(_, _) => todo!(),

        TyKind::Closure(_, _) => todo!(),

        TyKind::CoroutineClosure(_, _) => todo!(),

        TyKind::Coroutine(_, _) => todo!(),

        TyKind::CoroutineWitness(_, _) => todo!(),

        TyKind::Never => todo!(),

        TyKind::Tuple(_) => todo!(),

        TyKind::Alias(_, _) => todo!(),

        TyKind::Param(_) => todo!(),

        TyKind::Bound(_, _) => todo!(),

        TyKind::Placeholder(_) => todo!(),

        TyKind::Infer(_) => todo!(),

        TyKind::Error(_) => todo!()

    }
}
