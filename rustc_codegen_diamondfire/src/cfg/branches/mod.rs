//! Control Flow Graph Recovery

use rustc_hir::{ Body, Block };
use rustc_span::Span;

mod expr;
pub use expr::*;
mod stmt;
pub use stmt::*;


#[derive(Debug, Default)]
pub struct CfBranches {
    pub ifs : Vec<CfIfBranch>
}

#[derive(Debug, Default)]
pub struct CfIfBranch {
    pub cond_span : Span,
    pub has_else  : bool
}


pub fn find_body_cfb(hir : &Body<'_>) -> CfBranches {
    let mut branches = CfBranches::default();
    find_expr_cfb(&mut branches, hir.value);
    branches
}

pub fn find_block_cfb(branches : &mut CfBranches, block : &Block<'_>) {
    for stmt in block.stmts {
        find_stmt_cfb(branches, stmt);
    }
    if let Some(expr) = block.expr {
         find_expr_cfb(branches, expr);
    }
}
