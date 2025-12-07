use crate::diag;
use super::{
    CfBranches
};
use std::collections::BTreeMap;
use rustc_middle::{
    mir::{
        BasicBlocks,
        BasicBlock,
        TerminatorKind
    },
    ty::TyCtxt
};


#[derive(Default, Debug)]
pub struct CfaPrims {
    pub bbs : BTreeMap<BasicBlock, CfaPrim>
}

#[derive(Debug)]
pub enum CfaPrim {
    Jump {
        exit : BasicBlock
    },
    If {
        then : BasicBlock,
        exit : BasicBlock
    },
    IfElse {
        then : BasicBlock,
        els  : BasicBlock,
        exit : BasicBlock
    },
    Return,
    Unreachable
}

impl CfaPrim {
    pub fn exit(&self) -> Option<BasicBlock> { match (self) {
        Self::Jump { exit }       => Some(*exit),
        Self::If { exit, .. }     => Some(*exit),
        Self::IfElse { exit, .. } => Some(*exit),
        Self::Return              => None,
        Self::Unreachable         => None
    } }
}


pub fn analyse_cfb<'tcx>(
    tcx : TyCtxt<'tcx>,
    cfb : CfBranches,
    bbs : &BasicBlocks<'tcx>
) -> CfaPrims {
    let mut prims = CfaPrims::default();
    'bb_loop : for (bbi, bb,) in bbs.into_iter().enumerate() {
        let term = bb.terminator();
        match (&term.kind) {

            TerminatorKind::Goto { target } => {
                prims.bbs.insert(BasicBlock::from_usize(bbi), CfaPrim::Jump { exit : *target });
                continue 'bb_loop;
            },

            TerminatorKind::SwitchInt { discr, targets } => {
                for ifb in &cfb.ifs { if (ifb.cond_span == term.source_info.span) {
                    if (ifb.has_else) {
                        todo!()
                    } else {
                        let target_bbs = targets.all_targets();
                        assert_eq!(target_bbs.len(), 2); // then and fallback
                        prims.bbs.insert(BasicBlock::from_usize(bbi), CfaPrim::If { then : target_bbs[0], exit : target_bbs[1] });
                        continue 'bb_loop;
                    }
                } }
                todo!()
            },

            TerminatorKind::UnwindResume
            | TerminatorKind::UnwindTerminate(_)
            => { diag::unwinding_unsupported(tcx.dcx(), term.source_info.span); },

            TerminatorKind::Return => {
                prims.bbs.insert(BasicBlock::from_usize(bbi), CfaPrim::Return);
                continue 'bb_loop;
            },

            TerminatorKind::Unreachable => {
                prims.bbs.insert(BasicBlock::from_usize(bbi), CfaPrim::Unreachable);
                continue 'bb_loop;
            },

            TerminatorKind::Drop { place, target, unwind, replace, drop, async_fut } => todo!(),

            TerminatorKind::Call { func, args, destination, target, unwind, call_source, fn_span } => todo!(),

            TerminatorKind::TailCall { func, args, fn_span } => todo!(),

            TerminatorKind::Assert { cond, expected, msg, target, unwind } => todo!(),

            TerminatorKind::Yield { .. }
            | TerminatorKind::CoroutineDrop
            => { diag::coroutines_unsupported(tcx.dcx(), term.source_info.span); },

            TerminatorKind::FalseEdge { .. } => { unreachable!("disallowed after drop elaboration") },

            TerminatorKind::FalseUnwind { .. } => { unreachable!("disallowed after drop elaboration") },

            TerminatorKind::InlineAsm { .. } => { diag::inlineasm_unsupported(tcx.dcx(), term.source_info.span); }

        }
    }
    prims
}


struct Successors(Vec<BTreeMap<BasicBlock, usize>>);

fn bb_successors(
    bbs : &BasicBlocks<'_>
) -> Successors {
    let mut all_succs = Vec::new();
    for (bbi, bb,) in bbs.iter().enumerate() {
        let mut succs = BTreeMap::new();



        all_succs.push(succs);
    }
    Successors(all_succs)
}
