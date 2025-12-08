use super::{
    CfaPrims,
    CfaPrim
};
use core::fmt::{ self, Display, Formatter };
use rustc_middle::mir::BasicBlock;


#[derive(Default)]
pub struct CfrTree {
    pub groups : Vec<CfrTreeGroup>
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
        for group in &self.groups {
            if (f.alternate()) {
                write!(f, "\n{: >indent4$}", "", indent4 = 4*indent1)?;
            }
            group.fmt_indent(f, indent1)?;
            write!(f, ";{space}")?;
        }
        if (f.alternate() && ! self.groups.is_empty()) {
            write!(f, "\n{: >indent4$}", "", indent4 = 4*indent)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

pub enum CfrTreeGroup {
    Block(BasicBlock),
    If {
        then : CfrTree
    },
    IfElse {
        then : CfrTree,
        els  : CfrTree
    },
    Loop {
        then : CfrTree
    },
    While {
        cond : CfrTree,
        then : CfrTree
    },
    For {
        next   : BasicBlock,
        switch : BasicBlock,
        then   : CfrTree
    },
    Match {
        thens : Vec<CfrTree>
    },
    Return,
    Unreachable,
    Error
}
impl Display for CfrTreeGroup {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
impl CfrTreeGroup {
    fn fmt_indent(&self, f : &mut Formatter<'_>, indent : usize) -> fmt::Result {
        match (self) {
            Self::Block(bb) => {
                write!(f, "bb{}", bb.index())?;
            },
            Self::If { then } => {
                write!(f, "if (...) ")?;
                then.fmt_indent(f, indent)?;
            },
            Self::IfElse { then, els } => {
                write!(f, "if (...) ")?;
                then.fmt_indent(f, indent)?;
                write!(f, " else ")?;
                els.fmt_indent(f, indent)?;
            },
            Self::Loop { then } => {
                write!(f, "loop ")?;
                then.fmt_indent(f, indent)?;
            },
            Self::While { cond, then } => {
                write!(f, "while (")?;
                cond.fmt_indent(f, indent)?;
                write!(f, ") ")?;
                then.fmt_indent(f, indent)?;
            },
            Self::For { next, switch, then } => {
                write!(f, "for (bb{}; bb{};) ", next.index(), switch.index())?;
                then.fmt_indent(f, indent)?;
            },
            Self::Match { thens } => {
                write!(f, "match (...) {{")?;
                for (i, then,) in thens.iter().enumerate() {
                    if (i > 0) { write!(f, ",")?; }
                    write!(f, "\n{: >indent4$}... => ", "", indent4 = 4*indent)?;
                    then.fmt_indent(f, indent)?;
                }
                write!(f, "}}")?;
            },
            Self::Return      => { write!(f, "return")?; },
            Self::Unreachable => { write!(f, "unreachable")?; },
            Self::Error       => { write!(f, "error")?; }
        }
        Ok(())
    }
}

impl CfrTree {
    pub fn bb(bb : BasicBlock) -> Self {
        Self { groups : vec![ CfrTreeGroup::Block(bb) ] }
    }
    pub fn push(&mut self, g : CfrTreeGroup) {
        self.groups.push(g);
    }
    pub fn append(&mut self, tree : &mut Self) {
        self.groups.append(&mut tree.groups)
    }
}


pub fn recover_cfg(
    prims : &CfaPrims
) -> CfrTree {
    recover_cfg_node(prims, BasicBlock::ZERO, &mut Vec::new(), &mut Vec::new())
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum RecoveryScope {
    Any,
    Loop(BasicBlock)
}

fn recover_cfg_node(
    prims  : &CfaPrims,
    bb     : BasicBlock,
    until  : &mut Vec<BasicBlock>,
    scopes : &mut Vec<RecoveryScope>
) -> CfrTree {
    let     prim = prims.bbs.get(&bb).unwrap();
    let mut tree = CfrTree::default();
    match (prim) {

        CfaPrim::Sequence { .. } => { // TODO: Handle jumping to block starts.
            tree.push(CfrTreeGroup::Block(bb));
        },

        CfaPrim::If { then, exit } => {
            tree.push(CfrTreeGroup::Block(bb));
            if let Some(exit) = exit { until.push(*exit); }
            tree.push(CfrTreeGroup::If {
                then : recover_cfg_node(prims, *then, until, scopes)
            });
            if (exit.is_some()) { until.pop(); }
        },

        CfaPrim::IfElse { then, els, exit } => {
            tree.push(CfrTreeGroup::Block(bb));
            if let Some(exit) = exit { until.push(*exit); }
            tree.push(CfrTreeGroup::IfElse {
                then : recover_cfg_node(prims, *then, until, scopes),
                els  : recover_cfg_node(prims, *els, until, scopes)
            });
            if (exit.is_some()) { until.pop(); }
        },

        CfaPrim::LoopDelimiter { then } => {
            tree.push(CfrTreeGroup::Block(bb));
            if let Some(depth) = scopes.iter().position(|x| *x == RecoveryScope::Loop(*then)) {
                assert_eq!(depth, scopes.len() - 1);
            } else {
                until.push(*then);
                scopes.push(RecoveryScope::Loop(*then));
                tree.push(CfrTreeGroup::Loop {
                    then : recover_cfg_node(prims, *then, until, scopes)
                });
                _ = scopes.pop();
                _ = until.pop();
            }
        },

        CfaPrim::While { then, exit } => {
            until.extend([bb, *exit,]);
            scopes.push(RecoveryScope::Any);
            tree.push(CfrTreeGroup::While {
                cond : CfrTree::bb(bb),
                then : recover_cfg_node(prims, *then, until, scopes)
            });
            _ = scopes.pop();
            _ = until.pop();
            _ = until.pop();
        },

        CfaPrim::ForIter { iter } => {
            let CfaPrim::For { then, exit } = prims.bbs.get(iter).unwrap() else { unreachable!() };
            scopes.push(RecoveryScope::Any);
            until.extend([bb, *exit,]);
            tree.push(CfrTreeGroup::For {
                next   : bb,
                switch : *iter,
                then   : recover_cfg_node(prims, *then, until, scopes)
            });
            _ = scopes.pop();
            _ = until.pop();
            _ = until.pop();
            tree.append(&mut recover_cfg_node(prims, *exit, until, scopes))
        },

        CfaPrim::For { .. } => { unreachable!(); }, // Blocked by `CfaPrim::ForIter`.

        CfaPrim::Match { thens, exit } => {
            tree.push(CfrTreeGroup::Block(bb));
            if let Some(exit) = exit { until.push(*exit); }
            tree.push(CfrTreeGroup::Match { thens : thens.iter().map(|then| recover_cfg_node(prims, *then, until, scopes)).collect::<Vec<_>>() });
            if (exit.is_some()) { until.pop(); }
        },

        CfaPrim::Return => {
            tree.push(CfrTreeGroup::Block(bb));
            tree.push(CfrTreeGroup::Return);
        },

        CfaPrim::Unreachable => {
            tree.push(CfrTreeGroup::Block(bb));
            tree.push(CfrTreeGroup::Unreachable)
        },

        CfaPrim::Error => {
            tree.push(CfrTreeGroup::Error)
        }

    }
    if let Some(exit) = prim.exit() && (! until.contains(&exit)) {
        tree.append(&mut recover_cfg_node(prims, exit, until, scopes));
    }
    tree
}
