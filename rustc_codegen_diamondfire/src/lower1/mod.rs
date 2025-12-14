use crate::cfr::{ CfrTree, CfrBranch };
use bridgecg_diamondfire::mir::DfMirStmt;
use rustc_middle::{
    mir::Body,
    ty::TyCtxt
};

mod stmt;
use stmt::stmt_to_dfmir;

mod term;
use term::term_to_dfmir;


struct Lower1Ctx<'tcx> {
    tcx  : TyCtxt<'tcx>,
    body : &'tcx Body<'tcx>
}


pub fn mir_to_dfmir<'tcx>(
    tcx  : TyCtxt<'tcx>,
    body : &'tcx Body<'tcx>,
    cfr  : &CfrTree
) -> Vec<DfMirStmt> {
    let mut ctx = Lower1Ctx {
        tcx,
        body
    };
    let mut out = Vec::new();
    for branch in &cfr.branches {
        branch_to_dfmir(&mut ctx, branch, &mut out);
    }
    out
}


fn branch_to_dfmir(
    ctx    : &mut Lower1Ctx<'_>,
    branch : &CfrBranch,
    out    : &mut Vec<DfMirStmt>
) {
    match (branch) {

        CfrBranch::Block(bbi) => {
            let bb = ctx.body.basic_blocks.get(*bbi).unwrap();
            for stmt in &bb.statements {
                stmt_to_dfmir(ctx, stmt, out);
            }
            term_to_dfmir(ctx, bb.terminator(), out);
        },

        CfrBranch::Match { .. } => todo!(),

        CfrBranch::Loop { .. } => {
            // TODO
        },

        CfrBranch::Break { .. } => todo!(),

        CfrBranch::Continue { .. } => todo!(),

        CfrBranch::Return => {
            out.push(DfMirStmt::Return);
        },

        CfrBranch::Unreachable => { },

        CfrBranch::Todo => unreachable!()

    }
}
