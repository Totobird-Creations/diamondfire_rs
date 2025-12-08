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
    Sequence {
        exit : BasicBlock
    },
    If {
        then : BasicBlock,
        exit : Option<BasicBlock>
    },
    IfElse {
        then : BasicBlock,
        els  : BasicBlock,
        exit : Option<BasicBlock>
    },
    LoopDelimiter {
        then : BasicBlock
    },
    While {
        then : BasicBlock,
        exit : BasicBlock
    },
    Match {
        thens : Vec<BasicBlock>,
        exit  : Option<BasicBlock>
    },
    Return,
    Unreachable
}

impl CfaPrim {
    pub fn exit(&self) -> Option<BasicBlock> { match (self) {
        Self::Sequence { exit }    => Some(*exit),
        Self::If { exit, .. }      => *exit,
        Self::IfElse { exit, .. }  => *exit,
        Self::LoopDelimiter { .. } => None,
        Self::While { exit, .. }   => Some(*exit),
        Self::Match { exit, .. }   => *exit,
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

                for loopb in &cfb.loops {
                    // Loop
                    if (term.source_info.span.contains(loopb.kw_cond_span) && loopb.kw_cond_span.lo() == term.source_info.span.lo()) {
                        prims.bbs.insert(bbi, CfaPrim::LoopDelimiter { then : *target });
                        continue 'bb_loop;
                    }
                }

                // rustc_errors::Diag::<()>::new(tcx.dcx(), rustc_errors::Level::Error, format!("{:?} goto", bbi)).with_span(term.source_info.span).emit();

                prims.bbs.insert(bbi, CfaPrim::Sequence { exit : *target });
                continue 'bb_loop;
            },

            TerminatorKind::SwitchInt { targets, .. } => {

                for whileb in &cfb.whiles {
                    // While
                    if (whileb.kw_cond_span.contains(term.source_info.span)) {
                        assert_eq!(targets.all_values(), [0]); // exit
                        let target_bbs = targets.all_targets();
                        prims.bbs.insert(bbi, CfaPrim::While { then : target_bbs[1], exit : target_bbs[0] });
                        continue 'bb_loop;
                    }
                }

                for ifb in &cfb.ifs {
                    if (ifb.cond_span == term.source_info.span) {
                        // IfElse
                        if (ifb.has_else) {
                            assert_eq!(targets.all_values(), [0]); // else
                            let target_bbs = targets.all_targets();
                            println!();
                            prims.bbs.insert(bbi, {
                                if let Some((exit, _,)) = succs.find_reconvergence(target_bbs[1], target_bbs[0]) {
                                    CfaPrim::IfElse { then : target_bbs[1], els : target_bbs[0], exit : Some(exit) }
                                } else { CfaPrim::IfElse { then : target_bbs[1], els : target_bbs[0], exit : None } }
                            });
                        }
                        // If
                        else {
                            assert_eq!(targets.all_values(), [0]); // exit
                            let target_bbs = targets.all_targets();
                            prims.bbs.insert(bbi, {
                                if let Some((exit, _,)) = succs.find_reconvergence(target_bbs[1], target_bbs[0]) {
                                    if (exit == target_bbs[0]) { CfaPrim::If { then : target_bbs[1], exit : Some(exit) } }
                                    else { CfaPrim::IfElse { then : target_bbs[1], els : target_bbs[0], exit : Some(exit) } }
                                } else { CfaPrim::If { then : target_bbs[1], exit : None } }
                            });
                        }
                        continue 'bb_loop;
                    }
                }

                for matchb in &cfb.matches {
                    // Match
                    if (term.source_info.span.contains(matchb.kw_key_span) && matchb.kw_key_span.hi() == term.source_info.span.hi()) {
                        let target_bbs = targets.all_targets();
                        prims.bbs.insert(bbi, CfaPrim::Match {
                            thens : target_bbs.to_vec(),
                            exit  : succs.find_reconvergence_many(target_bbs)
                        });
                        continue 'bb_loop;
                    }
                }

                diag::missing_switch(tcx.dcx(), term.source_info.span);
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
            for (bbi1, depth1,) in skip {
                let depth2 = depth + depth1;
                if let Some(old_depth) = succs.get(&bbi) {
                    if (depth2 < *old_depth) { succs.insert(*bbi1, depth2); }
                } else { succs.insert(*bbi1, depth2); }
            }
        } else {
            if let Some(old_depth) = succs.get(&bbi) {
                if (depth < *old_depth) { succs.insert(bbi, depth); }
            } else { succs.insert(bbi, depth); }
            for succ in bbs.get(bbi).unwrap().terminator().successors() {
                if (! succs.contains_key(&succ)) {
                    Self::walk_bbs(all_succs, succs, bbs, succ, depth + 1);
                }
            }
        }
    }

}

impl Successors {

    fn find_reconvergence(&self, a : BasicBlock, b : BasicBlock) -> Option<(BasicBlock, usize,)> {
        let a_succs = self.0.get(&a).unwrap();
        let b_succs = self.0.get(&b).unwrap();
        a_succs.iter()
            .filter(|(bb, _,)| b_succs.contains_key(bb))
            .map(|(bb, _,)| (
                *bb,
                a_succs.get(bb).unwrap() + b_succs.get(bb).unwrap() // depth
            ))
            .min_by_key(|(_, depth,)| *depth)
    }

    fn find_reconvergence_many(&self, from : &[BasicBlock]) -> Option<BasicBlock> {
        let mut reconverge = None;
        for ai in 0..from.len() {
            for bi in (ai+1)..from.len() {
                let a = from[ai];
                let b = from[bi];
                if let Some((exit, _,)) = self.find_reconvergence(a, b) {
                    match (reconverge) {
                        Some(old_reconverge) => { if let Some((exit1, _,)) = self.find_reconvergence(old_reconverge, exit) {
                            reconverge = Some(exit1);
                        } },
                        None => { reconverge = Some(exit); }
                    }
                }
            }
        }
        reconverge
    }

}
