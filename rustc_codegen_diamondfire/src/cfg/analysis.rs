use crate::diag;
use super::{
    CfBranches
};
use std::collections::BTreeMap;
use rustc_middle::{
    mir::{
        BasicBlocks,
        BasicBlock,
        TerminatorKind,
        UnwindAction
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
    IfDiverge {
        then : BasicBlock
    },
    IfElse {
        then : BasicBlock,
        els  : BasicBlock,
        exit : BasicBlock
    },
    IfElseDiverge {
        then : BasicBlock,
        els  : BasicBlock
    },
    Return,
    Unreachable
}

impl CfaPrim {
    pub fn exit(&self) -> Option<BasicBlock> { match (self) {
        Self::Jump { exit }        => Some(*exit),
        Self::If { exit, .. }      => Some(*exit),
        Self::IfDiverge { .. }     => None,
        Self::IfElse { exit, .. }  => Some(*exit),
        Self::IfElseDiverge { .. } => None,
        Self::Return               => None,
        Self::Unreachable          => None
    } }
}


pub fn analyse_cfb<'tcx>(
    tcx : TyCtxt<'tcx>,
    cfb : CfBranches,
    bbs : &BasicBlocks<'tcx>
) -> CfaPrims {
    let succs = Successors::from(bbs);

    let mut prims = CfaPrims::default();
    'bb_loop : for bbi in bbs.indices() {
        let bb   = bbs.get(bbi).unwrap();
        let term = bb.terminator();
        match (&term.kind) {

            TerminatorKind::Goto { target } => {
                prims.bbs.insert(bbi, CfaPrim::Jump { exit : *target });
                continue 'bb_loop;
            },

            TerminatorKind::SwitchInt { targets, .. } => {

                for ifb in &cfb.ifs { if (ifb.cond_span == term.source_info.span) {
                    if (ifb.has_else) { // IfElse
                        todo!()
                    } else { // If
                        let target_bbs = targets.all_targets();
                        assert_eq!(target_bbs.len(), 2); // then and fallback
                        prims.bbs.insert(bbi, {
                            if let Some(exit) = succs.find_reconvergence(target_bbs[0], target_bbs[1]) {
                                if (exit == target_bbs[1]) { CfaPrim::If { then : target_bbs[0], exit } }
                                else { CfaPrim::IfElse { then : target_bbs[0], els : target_bbs[1], exit } }
                            } else { CfaPrim::IfDiverge { then : target_bbs[0] } }
                        });
                        continue 'bb_loop;
                    }
                } }

                todo!()

            },

            TerminatorKind::UnwindResume
            | TerminatorKind::UnwindTerminate(_)
            => { diag::unwinding_unsupported(tcx.dcx(), term.source_info.span); },

            TerminatorKind::Return => {
                prims.bbs.insert(bbi, CfaPrim::Return);
                continue 'bb_loop;
            },

            TerminatorKind::Unreachable => {
                prims.bbs.insert(bbi, CfaPrim::Unreachable);
                continue 'bb_loop;
            },

            TerminatorKind::Drop { target, drop, async_fut, .. } => {
                if (drop.is_some() || async_fut.is_some()) {
                    diag::coroutines_unsupported(tcx.dcx(), term.source_info.span);
                }
                todo!()
            },

            TerminatorKind::Call { target, unwind, .. } => {
                if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                    diag::unwinding_unsupported(tcx.dcx(), term.source_info.span);
                }
                todo!()
            },

            TerminatorKind::TailCall { .. } => todo!(),

            TerminatorKind::Assert { target, unwind, .. } => {
                if (! (*unwind == UnwindAction::Continue || *unwind == UnwindAction::Unreachable)) {
                    diag::unwinding_unsupported(tcx.dcx(), term.source_info.span);
                }
                todo!()
            },

            TerminatorKind::Yield { .. }
            | TerminatorKind::CoroutineDrop
            => { diag::coroutines_unsupported(tcx.dcx(), term.source_info.span); },

            TerminatorKind::FalseEdge { .. } => { diag::disallowed_post_drop_elaboration() },

            TerminatorKind::FalseUnwind { .. } => { diag::disallowed_post_drop_elaboration() },

            TerminatorKind::InlineAsm { .. } => { diag::inlineasm_unsupported(tcx.dcx(), term.source_info.span); }

        }
    }
    prims
}


struct Successors(BTreeMap<BasicBlock, BTreeMap<BasicBlock, usize>>);
impl Successors {

    fn from(bbs : &BasicBlocks<'_>) -> Self {
        let mut all_succs = BTreeMap::new();
        for bbi in bbs.indices() {
            let mut succs = BTreeMap::new();
            Self::walk_bbs(&all_succs, &mut succs, bbs, bbi, 0);
            all_succs.insert(bbi, succs);
        }
        Self(all_succs)
    }

    fn walk_bbs(
        all_succs : &BTreeMap<BasicBlock, BTreeMap<BasicBlock, usize>>,
        succs     : &mut BTreeMap<BasicBlock, usize>,
        bbs       : &BasicBlocks<'_>,
        bbi       : BasicBlock,
        depth     : usize
    ) {
        if let Some(skip) = all_succs.get(&bbi) {
            succs.extend(skip.iter().map(|(bbi1, depth1,)| (*bbi1, depth + depth1,)));
        } else {
            succs.insert(bbi, depth);
            for succ in bbs.get(bbi).unwrap().terminator().successors() {
                Self::walk_bbs(all_succs, succs, bbs, succ, depth + 1);
            }
        }
    }

}

impl Successors {
    fn find_reconvergence(&self, a : BasicBlock, b : BasicBlock) -> Option<BasicBlock> {
        let a_succs      = self.0.get(&a).unwrap();
        let b_succs      = self.0.get(&b).unwrap();
        a_succs.iter()
            .filter_map(|(a_succ, _,)| b_succs.contains_key(a_succ).then_some(*a_succ))
            .min_by_key(|bbi| a_succs.get(bbi).unwrap() + b_succs.get(bbi).unwrap())
    }
}
