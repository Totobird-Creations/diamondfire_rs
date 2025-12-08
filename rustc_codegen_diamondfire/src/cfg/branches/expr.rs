use super::{
    CfBranches,
    CfLoopBranch,
    CfIfBranch,
    CfMatchBranch,
    find_block_cfb
};
use rustc_hir::{
    Expr,
    ExprKind,
    LoopSource,
    MatchSource,
    StructTailExpr
};
use rustc_middle::ty::TyCtxt;


pub fn find_expr_cfb(tcx : &TyCtxt<'_>, branches : &mut CfBranches, expr : &Expr<'_>) { match (expr.kind) {

    ExprKind::ConstBlock(_)
    | ExprKind::Lit(_)
    | ExprKind::Path(_)
    | ExprKind::InlineAsm(_)
    | ExprKind::OffsetOf(_, _)
    => { },

    ExprKind::Array(values)
    | ExprKind::Tup(values)
    => {
        for value in values {
            find_expr_cfb(tcx, branches, value);
        }
    },

    ExprKind::Call(func, args)
    | ExprKind::MethodCall(_, func, args, _)
    => {
        find_expr_cfb(tcx, branches, func);
        for section in args {
            find_expr_cfb(tcx, branches, section);
        }
    },

    ExprKind::Use(value, _)
    | ExprKind::Unary(_, value)
    | ExprKind::Cast(value, _)
    | ExprKind::Type(value, _)
    | ExprKind::DropTemps(value)
    | ExprKind::AddrOf(_, _, value)
    | ExprKind::Become(value)
    | ExprKind::Repeat(value, _)
    | ExprKind::Yield(value, _)
    | ExprKind::UnsafeBinderCast(_, value, _)
    => {
        find_expr_cfb(tcx, branches, value);
    },

    ExprKind::Binary(_, a, b)
    | ExprKind::Assign(a, b, _)
    | ExprKind::AssignOp(_, a, b)
    => {
        find_expr_cfb(tcx, branches, a);
        find_expr_cfb(tcx, branches, b);
    },

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
        // TODO: label
        (match (source) {
            LoopSource::Loop    => { &mut branches.loops },
            LoopSource::While   => { &mut branches.whiles },
            LoopSource::ForLoop => todo!()
        }).push(CfLoopBranch { kw_cond_span : span, block_span : block.span });
        find_block_cfb(tcx, branches, block);
    },

    ExprKind::Match(key, arms, source) => {
        match (source) {
            MatchSource::Normal => {
                branches.matches.push(CfMatchBranch { kw_key_span : key.span });
            },
            MatchSource::Postfix => todo!(),
            MatchSource::ForLoopDesugar => { }, // TODO
            MatchSource::TryDesugar(_) => todo!(),
            MatchSource::AwaitDesugar => todo!(),
            MatchSource::FormatArgs => todo!(),
        }
        find_expr_cfb(tcx, branches, key);
        for arm in arms {
            if (arm.guard.is_some()) { todo!(); }
            find_expr_cfb(tcx, branches, arm.body);
        }
    },

    ExprKind::Closure(_) => todo!(),

    ExprKind::Block(block, label) => {
        if (label.is_some()) { todo!() }
        find_block_cfb(tcx, branches, block);
    },

    ExprKind::Field(base, _) => { find_expr_cfb(tcx, branches, base); },

    ExprKind::Index(base, index, _) => {
        { find_expr_cfb(tcx, branches, base); }
        { find_expr_cfb(tcx, branches, index); }
    },

    ExprKind::Break(dest, value,) => {
        if (dest.label.is_some()) { todo!() }
        if let Some(value) = value {
            find_expr_cfb(tcx, branches, value);
        }
        // TODO break

    },

    ExprKind::Continue(_) => todo!(),

    ExprKind::Ret(value) => {
        if let Some(value) = value {
            find_expr_cfb(tcx, branches, value);
        }
    },

    ExprKind::Struct(_, fields, tail) => {
        for field in fields {
            find_expr_cfb(tcx, branches, field.expr);
        }
        match (tail) {
            StructTailExpr::None             => { },
            StructTailExpr::Base(expr)       => { find_expr_cfb(tcx, branches, expr); },
            StructTailExpr::DefaultFields(_) => { },
        }
    },

    ExprKind::Err(_) => unreachable!()

} }
