use super::{
    CfgTree,
    CfgTreeKind,
    find_block_cfg,
    find_expr_cfg
};
use rustc_hir::{
    Stmt,
    StmtKind
};


pub fn find_stmt_cfg(stmt : &Stmt<'_>) -> Option<CfgTree> { match (stmt.kind) {

    StmtKind::Let(_) => todo!(),

    StmtKind::Item(_) => None,

    StmtKind::Expr(expr) => Some(find_expr_cfg(expr)),

    StmtKind::Semi(expr) => Some(find_expr_cfg(expr))

} }
