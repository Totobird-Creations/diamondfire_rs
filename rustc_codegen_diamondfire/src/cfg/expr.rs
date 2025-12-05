use super::{
    CfgTree,
    CfgTreeKind,
    find_block_cfg
};
use rustc_hir::{
    Expr,
    ExprKind
};


pub fn find_expr_cfg(expr : &Expr<'_>) -> CfgTree { match (expr.kind) {

    ExprKind::ConstBlock(_)
    | ExprKind::Lit(_)
    | ExprKind::Path(_)
    => CfgTree {
        span : expr.span,
        kind : CfgTreeKind::Simple
    },

    ExprKind::Array(_) => todo!(),

    ExprKind::Call(func, args) => {
        let mut sections = Vec::with_capacity(args.len() + 1);
        sections.push(func);
        sections.extend(args);
        sections.sort_by_key(|s| s.span);
        let mut seq = Vec::new();
        for section in sections {
            seq.push(find_expr_cfg(section));
        }
        CfgTree {
            span : expr.span,
            kind : CfgTreeKind::Seq(seq)
        }
    },

    ExprKind::MethodCall(_, _, _, _) => todo!(),

    ExprKind::Use(_, _) => todo!(),

    ExprKind::Tup(_) => todo!(),

    ExprKind::Binary(_, a, b)
    | ExprKind::AssignOp(_, a, b)
    => {
        let mut sections = [a, b];
        sections.sort_by_key(|s| s.span);
        let mut seq = Vec::new();
        for section in sections {
            seq.push(find_expr_cfg(section));
        }
        CfgTree {
            span : expr.span,
            kind : CfgTreeKind::Seq(seq)
        }
    },

    ExprKind::Unary(_, _) => todo!(),

    ExprKind::Cast(_, _) => todo!(),

    ExprKind::Type(_, _) => todo!(),

    ExprKind::DropTemps(_) => todo!(),

    ExprKind::Let(_) => todo!(),

    ExprKind::If(cond, then, els) => {
        CfgTree {
            span : expr.span,
            kind : CfgTreeKind::If(Box::new((
                find_expr_cfg(cond),
                find_expr_cfg(then),
                els.map(|els| find_expr_cfg(els))
            )))
        }
    }

    ExprKind::Loop(_, _, _, _) => todo!(),

    ExprKind::Match(_, _, _) => todo!(),

    ExprKind::Closure(_) => todo!(),

    ExprKind::Block(block, label) => {
        if (label.is_some()) { todo!() }
        find_block_cfg(block)
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
