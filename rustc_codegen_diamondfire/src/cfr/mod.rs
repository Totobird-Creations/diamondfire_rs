use rustc_data_structures::graph::{
    Predecessors,
    Successors
};
use rustc_middle::{
    mir::{
        BasicBlocks,
        BasicBlock,
        TerminatorKind
    },
    ty::TyCtxt
};


#[derive(Default, Debug)]
pub struct CfrTree {
    pub branches : Vec<CfrBranch>
}

#[derive(Debug)]
pub enum CfrBranch {
    Block(BasicBlock),
    If {
        cond : CfrTree,
        then : CfrTree
    },
    IfElse {
        cond : CfrTree,
        then : CfrTree,
        els  : CfrTree
    },
    While {
        cond : CfrTree,
        then : CfrTree
    },
    DoWhile {
        then : CfrTree,
        cond : CfrTree
    },
    Loop {
        then : CfrTree
    },
    Break {
        to : BasicBlock
    },
    Continue {
        from : BasicBlock
    },
    Return,
    Unreachable,
    Todo
}

#[derive(Debug)]
enum CfrScope {
    Loop {
        header : BasicBlock
    }
}


fn predecessors(body : &BasicBlocks, bb : BasicBlock) -> impl Iterator<Item = BasicBlock> {
    Predecessors::predecessors(body, bb)
}

fn successors(body : &BasicBlocks, bb : BasicBlock) -> impl Iterator<Item = BasicBlock> {
    Successors::successors(body, bb)
}

fn dominates(body : &BasicBlocks, a : BasicBlock, b : BasicBlock) -> bool {
    body.dominators().dominates(a, b)
}


pub fn find_cfr_tree(tcx : TyCtxt<'_>, body : &BasicBlocks<'_>) -> CfrTree {
    let mut scope = Vec::new();
    find_cfr_tree_at(tcx, body, &mut scope, BasicBlock::ZERO)
}

fn find_cfr_tree_at(tcx : TyCtxt<'_>, body : &BasicBlocks<'_>, scope : &mut Vec<CfrScope>, at : BasicBlock) -> CfrTree {

    // Loops
    let mut scope_pop = 0;
    for pbb in predecessors(body, at) {
        if (dominates(body, at, pbb)) {
            scope.push(CfrScope::Loop { header : at });
            scope_pop += 1;
            break;
        }
    }

    let term = body.get(at).unwrap().terminator();
    let tree = { match (&term.kind) {

        TerminatorKind::Goto { target }
        | TerminatorKind::Drop { target, .. }
        | TerminatorKind::Assert { target, .. }
        => {
            let mut branches = vec![ CfrBranch::Block(at) ];
            branches.append(&mut find_cfr_tree_at(tcx, body, scope, *target).branches);
            CfrTree { branches }
        },

        TerminatorKind::SwitchInt { discr, targets }
        => {
            CfrTree { branches : vec![ CfrBranch::Todo ] }
        },

        TerminatorKind::Return
        => { CfrTree { branches : vec![ CfrBranch::Block(at), CfrBranch::Return ] } },

        TerminatorKind::Unreachable
        | TerminatorKind::TailCall { .. }
        => { CfrTree { branches : vec![ CfrBranch::Block(at), CfrBranch::Unreachable ] } },

        TerminatorKind::Call { target, .. }
        => todo!(),

        TerminatorKind::UnwindResume
        | TerminatorKind::UnwindTerminate(_)
        | TerminatorKind::Yield { .. }
        | TerminatorKind::CoroutineDrop
        | TerminatorKind::FalseEdge { .. }
        | TerminatorKind::FalseUnwind { .. }
        | TerminatorKind::InlineAsm { .. }
        => { CfrTree::default() },

    } };

    for _ in 0..scope_pop {
        _ = scope.pop();
    }

    tree


    // if let Some(CfrScope::Loop { header }) = scope.last()
    //     && (term.successors().any(|s| s == *header))
    // {}

    // if let TerminatorKind::SwitchInt { targets, .. } = &term.kind {
    //     let target_bbs = targets.all_targets().iter().filter(|target_bb| {
    //         ! matches!(body.get(**target_bb).unwrap().terminator().kind, TerminatorKind::Unreachable)
    //     }).cloned().collect::<Vec<_>>();
    //     if (target_bbs.len() == 2 && dominates(body, bb, target_bbs[0]) && dominates(body, bb, target_bbs[1])) {
    //         println!("if(else) header {:?}", bb);
    //     }
    // }

}
