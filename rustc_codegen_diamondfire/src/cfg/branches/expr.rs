use super::{
    CfBranches,
    CfIfBranch,
    find_block_cfb
};
use rustc_hir::{
    Expr,
    ExprKind
};


pub fn find_expr_cfb(branches : &mut CfBranches, expr : &Expr<'_>) { match (expr.kind) {

    ExprKind::ConstBlock(_)
    | ExprKind::Lit(_)
    | ExprKind::Path(_)
    => { },

    ExprKind::Array(_) => todo!(),

    ExprKind::Call(func, args) => {
        find_expr_cfb(branches, func);
        for section in args {
            find_expr_cfb(branches, section);
        }
    },

    ExprKind::MethodCall(_, _, _, _) => todo!(),

    ExprKind::Use(_, _) => todo!(),

    ExprKind::Tup(_) => todo!(),

    ExprKind::Binary(_, a, b)
    | ExprKind::AssignOp(_, a, b)
    => {
        find_expr_cfb(branches, a);
        find_expr_cfb(branches, b);
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
        find_expr_cfb(branches, cond);
        find_expr_cfb(branches, then);
        if let Some(els) = els {
            find_expr_cfb(branches, els);
        }
    }

    ExprKind::Loop(_, _, _, _) => todo!(),

    ExprKind::Match(_, _, _) => todo!(),

    ExprKind::Closure(_) => todo!(),

    ExprKind::Block(block, label) => {
        if (label.is_some()) { todo!() }
        find_block_cfb(branches, block);
    },

    ExprKind::Assign(_, _, _) => todo!(),

    ExprKind::Field(_, _) => todo!(),

    ExprKind::Index(_, _, _) => todo!(),

    ExprKind::AddrOf(_, _, _) => todo!(),

    ExprKind::Break(_, _) => todo!(),

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
