use rustc_errors::{
    DiagCtxtHandle,
    Diag,
    Level,
    ErrorGuaranteed
};
use rustc_span::Span;


#[inline(always)]
#[track_caller]
pub fn disallowed_post_drop_elaboration() {
    unreachable!("disallowed after drop elaboration")
}

#[inline(always)]
#[track_caller]
pub fn disallowed_post_coroutine_lowering() {
    unreachable!("disallowed after coroutine lowering")
}

pub fn unions_unsupported(dcx : DiagCtxtHandle<'_>, span : Span) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        "unions are currently unsupported by the `diamondfire-unknown-unknown` target"
    ).with_span(span).emit();
}

pub fn unwinding_unsupported(dcx : DiagCtxtHandle<'_>, span : Span) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        "unwinding is currently unsupported by the `diamondfire-unknown-unknown` target"
    ).with_span(span).emit();
}

pub fn coroutines_unsupported(dcx : DiagCtxtHandle<'_>, span : Span) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        "coroutines are currently unsupported by the `diamondfire-unknown-unknown` target"
    ).with_span(span).emit();
}

pub fn intrinsic_unsupported(dcx : DiagCtxtHandle<'_>, span : Span, name : &str) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        format!("the `{}` intrinsic is currently unsupported by the `diamondfire-unknown-unknown` target", name)
    ).with_span(span).emit();
}

pub fn inlineasm_unsupported(dcx : DiagCtxtHandle<'_>, span : Span) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        "inline assembly is unsupported by the `diamondfire-unknown-unknown` target"
    ).with_span(span).emit();
}

pub fn globalasm_unsupported(dcx : DiagCtxtHandle<'_>, span : Span) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        "global assembly is unsupported by the `diamondfire-unknown-unknown` target"
    ).with_span(span).emit();
}
