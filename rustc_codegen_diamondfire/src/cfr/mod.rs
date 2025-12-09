use core::fmt::{ self, Display, Formatter };
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
impl Display for CfrTree {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
impl CfrTree {
    fn fmt_indent(&self, f : &mut Formatter<'_>, indent : usize) -> fmt::Result {
        let indent1 = indent + 1;
        let space   = if (f.alternate()) { "" } else { " " };
        write!(f, "{{{space}")?;
        for branch in &self.branches {
            if (f.alternate()) {
                write!(f, "\n{: >indent4$}", "", indent4 = 4*indent1)?;
            }
            branch.fmt_indent(f, indent1)?;
            write!(f, ";{space}")?;
        }
        if (f.alternate() && ! self.branches.is_empty()) {
            write!(f, "\n{: >indent4$}", "", indent4 = 4*indent)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}


#[derive(Debug)]
pub enum CfrBranch {
    Block(BasicBlock),
    Match {
        cases : Vec<CfrTree>
    },
    Loop {
        /// First branch must be `CfrBranch::Block` with the loop header.
        body : CfrTree
    },
    // Break {
    //     to : BasicBlock
    // },
    Continue {
        header : BasicBlock
    },
    Return,
    Unreachable,
    Todo
}
impl Display for CfrBranch {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
impl CfrBranch {
    fn fmt_indent(&self, f : &mut Formatter<'_>, indent : usize) -> fmt::Result {
        match (self) {
            Self::Block(bb) => {
                write!(f, "bb{}", bb.index())?;
            },
            Self::Match { cases } => {
                if (cases.len() == 2) {
                    write!(f, "if (...) ")?;
                    cases[0].fmt_indent(f, indent)?;
                    write!(f, " else ")?;
                    cases[1].fmt_indent(f, indent)?;
                } else {
                    write!(f, "match (...) {{")?;
                    for (i, case,) in cases.iter().enumerate() {
                        if (i > 0) { write!(f, ",")?; }
                        write!(f, "\n{: >indent4$}... => ", "", indent4 = 4*(indent+1))?;
                        case.fmt_indent(f, indent+1)?;
                    }
                    write!(f, "\n{: >indent4$}}}", "", indent4 = 4*indent)?;
                }
            },
            Self::Loop { body } => {
                write!(f, "loop ")?;
                body.fmt_indent(f, indent)?;
            },
            Self::Continue { header } => {
                write!(f, "continue -> bb{}", header.index())?;
            },
            Self::Return      => { write!(f, "return")?; },
            Self::Unreachable => { write!(f, "unreachable")?; },
            Self::Todo        => { write!(f, "todo")?; }
        }
        Ok(())
    }
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
    let mut tree_root = CfrTree::default();
    let mut tree      = &mut tree_root;

    if let Some(CfrScope::Loop { header }) = scope.last() {
        if (*header == at) {
            tree.branches.push(CfrBranch::Continue { header : *header });
            return tree_root
        }
        // TODO: Break
    }

    // Loops
    let mut scope_pop = 0;
    for pbb in predecessors(body, at) {
        if (dominates(body, at, pbb)) {
            scope.push(CfrScope::Loop { header : at });
            scope_pop += 1;
            tree.branches.push(CfrBranch::Loop { body : CfrTree::default() });
            let CfrBranch::Loop { body } = tree.branches.last_mut().unwrap() else { unreachable!() };
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
            tree.branches.append(&mut find_cfr_tree_at(tcx, body, scope, *target).branches);
        },

        TerminatorKind::SwitchInt { targets, .. }
        => {
            let target_bbs = targets.all_targets().iter().filter(|target_bb| {
                ! matches!(body.get(**target_bb).unwrap().terminator().kind, TerminatorKind::Unreachable)
            }).cloned().collect::<Vec<_>>();

            tree.branches.push(CfrBranch::Block(at));
            if (target_bbs.iter().all(|target_bb| dominates(body, at, *target_bb))) {
                tree.branches.push(CfrBranch::Match {
                    cases : target_bbs.iter()
                        .map(|target_bb| find_cfr_tree_at(tcx, body, scope, *target_bb))
                        .collect::<Vec<_>>()
                });
            } else {
                todo!()
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
                tree.branches.append(&mut find_cfr_tree_at(tcx, body, scope, *target).branches);
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
        _ = scope.pop();
    }

    tree_root
}
