use std::collections::BTreeMap;
use rustc_data_structures::graph::Predecessors;
use rustc_middle::mir::{
    BasicBlocks,
    BasicBlock
};


pub fn predecessors(body : &BasicBlocks, bb : BasicBlock) -> impl Iterator<Item = BasicBlock> {
    Predecessors::predecessors(body, bb)
}

pub fn dominates(body : &BasicBlocks, a : BasicBlock, b : BasicBlock) -> bool {
    body.dominators().dominates(a, b)
}


pub struct Successors(BTreeMap<BasicBlock, BTreeMap<BasicBlock, usize>>);
impl Successors {

    pub fn from(body : &BasicBlocks<'_>) -> Self {
        let mut all_succs = BTreeMap::new();
        for bb in body.indices() {
            let mut succs = BTreeMap::new();
            Self::walk_bbs(&all_succs, &mut succs, body, bb, 0);
            all_succs.insert(bb, succs);
        }
        Self(all_succs)
    }

    pub fn walk_bbs(
        all_succs : &BTreeMap<BasicBlock, BTreeMap<BasicBlock, usize>>,
        succs     : &mut BTreeMap<BasicBlock, usize>,
        body      : &BasicBlocks<'_>,
        bb        : BasicBlock,
        depth     : usize
    ) {
        if let Some(skip) = all_succs.get(&bb) {
            for (bb1, depth1,) in skip {
                let depth2 = depth + depth1;
                if let Some(old_depth) = succs.get(&bb) {
                    if (depth2 < *old_depth) { succs.insert(*bb1, depth2); }
                } else { succs.insert(*bb1, depth2); }
            }
        } else {
            if let Some(old_depth) = succs.get(&bb) {
                if (depth < *old_depth) { succs.insert(bb, depth); }
            } else { succs.insert(bb, depth); }
            for succ in body.get(bb).unwrap().terminator().successors() {
                if (! succs.contains_key(&succ)) {
                    Self::walk_bbs(all_succs, succs, body, succ, depth + 1);
                }
            }
        }
    }

}

impl Successors {

    pub fn find_reconvergence(&self, a : BasicBlock, b : BasicBlock) -> Option<(BasicBlock, usize,)> {
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

    pub fn find_reconvergence_many(&self, from : &[BasicBlock]) -> Option<BasicBlock> {
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
