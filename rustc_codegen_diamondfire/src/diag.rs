use rustc_errors::{
    DiagCtxtHandle,
    Diag,
    Level,
    ErrorGuaranteed
};
use rustc_span::Span;


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

pub fn inlineasm_unsupported(dcx : DiagCtxtHandle<'_>, span : Span) {
    Diag::<ErrorGuaranteed>::new(dcx,
        Level::Error,
        "inline assembly is unsupported by the `diamondfire-unknown-unknown` target"
    ).with_span(span).emit();
}
