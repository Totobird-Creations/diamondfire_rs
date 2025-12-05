//! Control Flow Graph Recovery

use rustc_hir::{ Body, Block };
use rustc_span::Span;

mod expr;
pub use expr::*;
mod stmt;
pub use stmt::*;

pub mod recover;


#[derive(Debug)]
pub struct CfgTree {
    pub span : Span,
    pub kind : CfgTreeKind
}

#[derive(Debug)]
pub enum CfgTreeKind {
    Simple,
    /// Consecutive branches
    Seq(Vec<CfgTree>),
    /// Condition, Then, Else
    If(Box<(CfgTree, CfgTree, Option<CfgTree>)>)
}


pub fn find_body_cfg(hir : &Body<'_>) -> CfgTree {
    find_expr_cfg(hir.value)
}

pub fn find_block_cfg(block : &Block<'_>) -> CfgTree {
    let mut seq = Vec::with_capacity(block.stmts.len() + 1);
    for stmt in block.stmts {
        if let Some(branch) = find_stmt_cfg(stmt) {
            seq.push(branch);
        }
    }
    if let Some(expr) = block.expr {
         seq.push(find_expr_cfg(expr));
    }
    CfgTree {
        span : block.span,
        kind : CfgTreeKind::Seq(seq)
    }
}
