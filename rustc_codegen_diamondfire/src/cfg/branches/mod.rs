//! Control Flow Graph Recovery

use rustc_hir::{ Body, Block };
use rustc_span::Span;
use rustc_middle::ty::TyCtxt;

mod expr;
pub use expr::*;
mod stmt;
pub use stmt::*;


#[derive(Debug, Default)]
pub struct CfBranches {
    pub loops   : Vec<CfLoopBranch>,
    pub whiles  : Vec<CfLoopBranch>,
    pub fors    : Vec<CfLoopBranch>,
    pub ifs     : Vec<CfIfBranch>,
    pub matches : Vec<CfMatchBranch>
}

#[derive(Debug)]
pub struct CfLoopBranch {
    pub kw_cond_span : Span,
    pub block_span   : Span
}

#[derive(Debug)]
pub struct CfIfBranch {
    pub cond_span : Span,
    pub has_else  : bool
}

#[derive(Debug)]
pub struct CfMatchBranch {
    pub kw_key_span : Span
}


pub fn find_body_cfb(tcx : &TyCtxt<'_>, hir : &Body<'_>) -> CfBranches {
    let mut branches = CfBranches::default();
    find_expr_cfb(tcx, &mut branches, hir.value);
    branches
}

pub fn find_block_cfb(tcx : &TyCtxt<'_>, branches : &mut CfBranches, block : &Block<'_>) {
    for stmt in block.stmts {
        find_stmt_cfb(tcx, branches, stmt);
    }
    if let Some(expr) = block.expr {
         find_expr_cfb(tcx, branches, expr);
    }
}
