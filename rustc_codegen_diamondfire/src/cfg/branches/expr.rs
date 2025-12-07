use super::{
    CfBranches,
    CfWhileBranch,
    CfIfBranch,
    find_block_cfb
};
use rustc_hir::{
    Expr,
    ExprKind,
    LoopSource
};
use rustc_middle::ty::TyCtxt;


pub fn find_expr_cfb(tcx : &TyCtxt<'_>, branches : &mut CfBranches, expr : &Expr<'_>) { match (expr.kind) {

    ExprKind::ConstBlock(_)
    | ExprKind::Lit(_)
    | ExprKind::Path(_)
    => { },

    ExprKind::Array(_) => todo!(),

    ExprKind::Call(func, args) => {
        find_expr_cfb(tcx, branches, func);
        for section in args {
            find_expr_cfb(tcx, branches, section);
        }
    },

    ExprKind::MethodCall(_, _, _, _) => todo!(),

    ExprKind::Use(_, _) => todo!(),

    ExprKind::Tup(_) => todo!(),

    ExprKind::Binary(_, a, b)
    | ExprKind::AssignOp(_, a, b)
    => {
        find_expr_cfb(tcx, branches, a);
        find_expr_cfb(tcx, branches, b);
    },

    ExprKind::Unary(_, _) => todo!(),

    ExprKind::Cast(_, _) => todo!(),

    ExprKind::Type(_, _) => todo!(),

    ExprKind::DropTemps(_) => todo!(),

    ExprKind::Let(_) => todo!(),

    ExprKind::If(cond, then, els) => {
        branches.ifs.push(CfIfBranch {
            cond_span : cond.span,
            has_else  : els.is_some()
        });
        find_expr_cfb(tcx, branches, cond);
        find_expr_cfb(tcx, branches, then);
        if let Some(els) = els {
            find_expr_cfb(tcx, branches, els);
        }
    }

    ExprKind::Loop(block, label, source, span) => {
        if (label.is_some()) { todo!() }
        match (source) {
            LoopSource::Loop => todo!(),
            LoopSource::While => {
                branches.whiles.push(CfWhileBranch {
                    kw_cond_span : span
                });
            },
            LoopSource::ForLoop => todo!(),
        }
        find_block_cfb(tcx, branches, block);
    },

    ExprKind::Match(_, _, _) => todo!(),

    ExprKind::Closure(_) => todo!(),

    ExprKind::Block(block, label) => {
        if (label.is_some()) { todo!() }
        find_block_cfb(tcx, branches, block);
    },

    ExprKind::Assign(_, _, _) => todo!(),

    ExprKind::Field(_, _) => todo!(),

    ExprKind::Index(_, _, _) => todo!(),

    ExprKind::AddrOf(_, _, _) => todo!(),

    ExprKind::Break(dest, value,) => {
        if (dest.label.is_some()) { todo!() }
        if let Some(value) = value {
            find_expr_cfb(tcx, branches, value);
        }
        // TODO break

    },

    ExprKind::Continue(_) => todo!(),

    ExprKind::Ret(_) => todo!(),

    ExprKind::Become(_) => todo!(),

    ExprKind::InlineAsm(_) => todo!(),

    ExprKind::OffsetOf(_, _) => todo!(),

    ExprKind::Struct(_, _, _) => todo!(),

    ExprKind::Repeat(_, _) => todo!(),

    ExprKind::Yield(_, _) => todo!(),

    ExprKind::UnsafeBinderCast(_, _, _) => todo!(),

    ExprKind::Err(_) => todo!()

} }
