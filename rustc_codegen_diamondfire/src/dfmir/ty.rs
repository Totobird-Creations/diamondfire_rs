use rustc_middle::ty::{
    TyCtxt,
    TyKind,
    IntTy,
    UintTy,
    GenericArg,
    FnSigTys
};
use rustc_span::def_id::DefId;


pub enum DfMirTy<'tcx> {

    /// Boolean.
    Bool,

    /// Single-character.
    Char,

    /// 8-bit signed integer.
    I8,
    /// 16-bit signed integer.
    I16,
    /// 32-bit signed integer.
    I32,
    /// Oversized signed integer. Value will be kept in the range `(i64::MIN/1000)..(i64::MAX/1000)`.
    ILarge,

    /// 8-bit unsigned integer.
    U8,
    /// 16-bit unsigned integer.
    U16,
    /// 32-bit unsigned integer.
    U32,
    /// Oversized unsigned integer. Value will be kept in the range `0..(i64::MAX/1000)`.
    ULarge,

    /// 64-bit fixed-point number with three decimal places of precision.
    FFixed,

    /// Not a valid value. Only used during lowering.
    StrPointee,
    /// Not a valid value. Only used during lowering.
    SlicePointee(Box<DfMirTy<'tcx>>),
    /// Not a valid value. Only used during lowering.
    FnPointee(DefId, Vec<GenericArg<'tcx>>),

    /// `&str` in rsMIR, string in dfMIR.
    /// May only be used for references backed by immutable memory.
    StrPointed,
    /// `&[T]` in rsMIR, list in dfMIR.
    /// May only be used for references backed by immutable memory.
    SlicePointed(Box<DfMirTy<'tcx>>),

    /// A pointer to a value.
    Pointer(Box<DfMirTy<'tcx>>),
    /// A pointer to a function.
    FnPointer(FnSigTys<TyCtxt<'tcx>>)

}

impl<'tcx> From<&TyKind<'tcx>> for DfMirTy<'tcx> {
    fn from(tyk : &TyKind<'tcx>) -> Self {
        match (tyk) {
            TyKind::Bool => Self::Bool,
            TyKind::Char => Self::Char,
            TyKind::Int(ty) => { match (ty) {
                IntTy::I8                 => Self::I8,
                IntTy::I16                => Self::I16,
                IntTy::I32 | IntTy::Isize => Self::I32,
                IntTy::I64 | IntTy::I128  => Self::ILarge
            } },
            TyKind::Uint(ty) => { match (ty) {
                UintTy::U8                  => Self::U8,
                UintTy::U16                 => Self::U16,
                UintTy::U32 | UintTy::Usize => Self::U32,
                UintTy::U64 | UintTy::U128  => Self::ULarge
            } },
            TyKind::Float(_)           => Self::FFixed,
            TyKind::Adt(_, _)          => todo!(),
            TyKind::Foreign(_)         => todo!(),
            TyKind::Str                => Self::StrPointee,
            TyKind::Array(_, _)        => todo!(),
            TyKind::Pat(_, _)          => todo!(),
            TyKind::Slice(ty)          => Self::SlicePointee(Box::new(DfMirTy::from(ty.kind()))),
            TyKind::RawPtr(ty, _,)     => Self::Pointer(Box::new(DfMirTy::from(ty.kind()))),
            TyKind::Ref(_, ty, is_mut) => {
                if (is_mut.is_not()) { match (ty.kind()) {
                    TyKind::Str       => Self::StrPointed,
                    TyKind::Slice(ty) => Self::SlicePointed(Box::new(DfMirTy::from(ty.kind()))),
                    tyk               => Self::Pointer(Box::new(DfMirTy::from(tyk)))
                } } else { Self::Pointer(Box::new(DfMirTy::from(tyk))) }
            },
            TyKind::FnDef(def_id, generics) => Self::FnPointee(*def_id, generics.to_vec()),
            TyKind::FnPtr(sig, _)           => Self::FnPointer(sig.skip_binder()),
            TyKind::UnsafeBinder(_)         => todo!(),
            TyKind::Dynamic(_, _)           => todo!(),
            TyKind::Closure(_, _)           => todo!(),
            TyKind::CoroutineClosure(_, _)  => todo!(),
            TyKind::Coroutine(_, _)         => todo!(),
            TyKind::CoroutineWitness(_, _)  => todo!(),
            TyKind::Never                   => unreachable!(),
            TyKind::Tuple(_)                => todo!(),
            TyKind::Alias(_, _)             => todo!(),
            TyKind::Param(_)                => unreachable!(),
            TyKind::Bound(_, _)             => todo!(),
            TyKind::Placeholder(_)          => unreachable!(),
            TyKind::Infer(_)                => unreachable!(),
            TyKind::Error(_)                => unreachable!()
        }
    }
}
