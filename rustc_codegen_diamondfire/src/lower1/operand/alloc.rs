use crate::diag;
use super::{
    Lower1Ctx,
    scalar_int_to_dfmir
};
use bridgecg_diamondfire::dfmir::{
    DfMirStmt,
    DfMirTemporary
};
use rustc_abi::{
    Size,
    VariantIdx
};
use rustc_middle::{
    mir::interpret::{
        Allocation,
        AllocRange,
        Scalar
    },
    ty::{
        Ty,
        TyKind,
        TypingEnv,
        AdtKind
    }
};
use rustc_span::Span;


pub fn alloc_to_dfmir<'tcx>(
    ctx    : &mut Lower1Ctx<'tcx, '_>,
    ty     : Ty<'tcx>,
    alloc  : &Allocation,
    offset : Size,
    out    : &mut Vec<DfMirStmt>,
    span   : Span
) -> DfMirTemporary {
    let pci    = TypingEnv::fully_monomorphized().as_query_input(ty);
    let layout = ctx.tcx.layout_of(pci).unwrap();

    match (ty.kind()) {

        TyKind::Bool|TyKind::Int(_)|TyKind::Uint(_)|TyKind::Float(_) => {
            let range  = AllocRange { start : offset, size : layout.size };
            let Scalar::Int(scalar_int) = alloc.read_scalar(&ctx.tcx.data_layout, range, false).unwrap()
                else { unreachable!() };
            scalar_int_to_dfmir(ctx, ty, scalar_int, out)
        },

        TyKind::Adt(adt_def, generics) => { match (adt_def.adt_kind()) {

            AdtKind::Struct => {
                let mut field_temps = Vec::new();
                let variant = adt_def.variant(VariantIdx::from_usize(0));
                for (i, field_def) in variant.fields.iter().enumerate() {
                    let field_ty     = field_def.ty(ctx.tcx, generics);
                    let field_offset = layout.fields.offset(i);
                    field_temps.push(alloc_to_dfmir(ctx, field_ty, alloc, offset + field_offset, out, span));
                }
                let dst = ctx.next_temp();
                out.push(DfMirStmt::TStruct { dst, fields : field_temps });
                dst
            },

            AdtKind::Enum => {
                todo!()
            },

            AdtKind::Union => {
                diag::unions_unsupported(ctx.tcx.dcx(), span);
                DfMirTemporary::PLACEHOLDER
            }

        } },

        tyk => unimplemented!("{:?}", tyk)
    }
}
