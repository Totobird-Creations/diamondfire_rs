use rustc_middle::mir::{
    BasicBlocks,
    BasicBlock,
    TerminatorKind
};

mod tree;
pub use tree::*;

mod relation;
use relation::*;

#[derive(Debug)]
enum CfrScope {
    Match {
        header : BasicBlock,
        exit   : Option<BasicBlock>
    },
    Loop {
        header : BasicBlock,
        exit   : Option<BasicBlock>
    }
}


pub fn find_cfr_tree(body : &BasicBlocks<'_>) -> CfrTree {
    let mut scopes = Vec::new();
    let     succs  = Successors::from(body);
    find_cfr_tree_at(body,
        &succs,
        &mut scopes,
        BasicBlock::ZERO
    )
}

fn find_cfr_tree_at(body : &BasicBlocks<'_>,
    succs  : &Successors,
    scopes : &mut Vec<CfrScope>,
    at     : BasicBlock
) -> CfrTree {
    let mut tree_root = CfrTree::default();
    let mut tree      = &mut tree_root;

    for scope in scopes.iter().rev() { match (scope) {
        CfrScope::Match { header, exit : Some(exit) } if (*exit == at) => {
            tree.branches.push(CfrBranch::Break { header : *header });
            return tree_root;
        },
        CfrScope::Loop { header, exit } => {
            if (*header == at) {
                tree.branches.push(CfrBranch::Continue { header : *header });
                return tree_root;
            } else if (*exit == Some(at)) {
                tree.branches.push(CfrBranch:: Break { header : *header });
                return tree_root;
            }
        },
        _ => { }
        // TODO: Break
    } }

    // Loops
    let mut scope_pop = 0;
    for pbb in predecessors(body, at) {
        if (dominates(body, at, pbb)) {
            let exit = None; // TODO: Find loop exit for breaks.
            scopes.push(CfrScope::Loop { header : at, exit });
            scope_pop += 1;
            tree.branches.push(CfrBranch::Loop {
                header : at,
                body   : CfrTree::default()
            });
            let CfrBranch::Loop { body, .. } = tree.branches.last_mut().unwrap()
                else { unreachable!() };
            tree = body;
            break;
        }
    }

    let term = body.get(at).unwrap().terminator();
    match (&term.kind) {

        TerminatorKind::Goto { target }
        | TerminatorKind::Drop { target, .. }
        | TerminatorKind::Assert { target, .. }
        => {
            tree.branches.push(CfrBranch::Block(at));
            tree.branches.append(&mut find_cfr_tree_at(body, succs, scopes, *target).branches);
        },

        TerminatorKind::SwitchInt { targets, .. }
        => {
            tree.branches.push(CfrBranch::Block(at));

            if (targets.all_targets().iter().filter(|target_bb| {
                ! matches!(body.get(**target_bb).unwrap().terminator().kind, TerminatorKind::Unreachable)
            }).all(|target_bb| dominates(body, at, *target_bb))) {

                let exit = succs.find_reconvergence_many(targets.all_targets());
                scopes.push(CfrScope::Match { header : at, exit });
                tree.branches.push(CfrBranch::Match {
                    header   : at,
                    cases    : targets.all_values().iter()
                        .zip(targets.all_targets())
                        .map(|(key, target_bb,)| (key.get(), find_cfr_tree_at(body, succs, scopes, *target_bb),))
                        .collect::<Vec<_>>(),
                    fallback : find_cfr_tree_at(body, succs, scopes, *targets.all_targets().last().unwrap())
                });
                scopes.pop();
                if let Some(exit) = exit {
                    tree.branches.append(&mut find_cfr_tree_at(body, succs, scopes, exit).branches);
                }

            } else {
                tree.branches.push(CfrBranch::Todo); // TODO
            }
        },

        TerminatorKind::Return
        => {
            tree.branches.push(CfrBranch::Block(at));
            tree.branches.push(CfrBranch::Return);
        },

        TerminatorKind::Unreachable
        | TerminatorKind::TailCall { .. }
        => {
            tree.branches.push(CfrBranch::Block(at));
            tree.branches.push(CfrBranch::Unreachable);
        },

        TerminatorKind::Call { target, .. }
        => {
            tree.branches.push(CfrBranch::Block(at));
            if let Some(target) = target {
                tree.branches.append(&mut find_cfr_tree_at(body, succs, scopes, *target).branches);
            } else {
                tree.branches.push(CfrBranch::Unreachable);
            }
        },

        TerminatorKind::UnwindResume
        | TerminatorKind::UnwindTerminate(_)
        | TerminatorKind::Yield { .. }
        | TerminatorKind::CoroutineDrop
        | TerminatorKind::FalseEdge { .. }
        | TerminatorKind::FalseUnwind { .. }
        | TerminatorKind::InlineAsm { .. }
        => { },

    };

    for _ in 0..scope_pop {
        _ = scopes.pop();
    }

    tree_root
}
