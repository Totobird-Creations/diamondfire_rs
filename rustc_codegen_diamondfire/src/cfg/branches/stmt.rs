use super::{
    CfBranches,
    find_expr_cfb
};
use rustc_hir::{
    Stmt,
    StmtKind
};
use rustc_middle::ty::TyCtxt;


pub fn find_stmt_cfb(tcx : &TyCtxt<'_>, branches : &mut CfBranches, stmt : &Stmt<'_>) { match (stmt.kind) {

    StmtKind::Let(stmt) => {
        if (stmt.els.is_some()) { todo!(); }
        if let Some(init) = &stmt.init {
            find_expr_cfb(tcx, branches, init);
        }
    },

    StmtKind::Item(_) => { },

    StmtKind::Expr(expr) => find_expr_cfb(tcx, branches, expr),

    StmtKind::Semi(expr) => find_expr_cfb(tcx, branches, expr)

} }
