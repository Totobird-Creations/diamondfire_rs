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
    While {
        cond : CfrTree,
        then : CfrTree
    },
    Return,
    Unreachable
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
            Self::While { cond, then } => {
                write!(f, "while (")?;
                cond.fmt_indent(f, indent)?;
                write!(f, ") ")?;
                then.fmt_indent(f, indent)?;
            },
            Self::Return      => { write!(f, "return")?; },
            Self::Unreachable => { write!(f, "unreachable")?; }
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
    recover_cfg_node(prims, BasicBlock::ZERO, &mut Vec::new())
}


fn recover_cfg_node(
    prims : &CfaPrims,
    bb    : BasicBlock,
    until : &mut Vec<BasicBlock>
) -> CfrTree {
    let     prim = prims.bbs.get(&bb).unwrap();
    let mut tree = CfrTree::default();
    match (prim) {

        CfaPrim::Sequence { .. } => { // TODO: Handle jumping to block starts.
            tree.push(CfrTreeGroup::Block(bb));
        },

        CfaPrim::If { then, exit } => {
            tree.push(CfrTreeGroup::Block(bb));
            until.push(*exit);
            tree.push(CfrTreeGroup::If {
                then : recover_cfg_node(prims, *then, until)
            });
            _ = until.pop();
        },

        CfaPrim::IfDiverge { then } => {
            tree.push(CfrTreeGroup::Block(bb));
            tree.push(CfrTreeGroup::If {
                then : recover_cfg_node(prims, *then, until)
            });
        },

        CfaPrim::IfElse { then, els, exit } => {
            tree.push(CfrTreeGroup::Block(bb));
            until.push(*exit);
            tree.push(CfrTreeGroup::IfElse {
                then : recover_cfg_node(prims, *then, until),
                els  : recover_cfg_node(prims, *els, until)
            });
            _ = until.pop();
        },

        CfaPrim::IfElseDiverge { then, els } => {
            tree.push(CfrTreeGroup::Block(bb));
            tree.push(CfrTreeGroup::IfElse {
                then : recover_cfg_node(prims, *then, until),
                els  : recover_cfg_node(prims, *els, until)
            });
        },

        CfaPrim::While { then, exit } => {
            until.extend([bb, *exit,]);
            tree.push(CfrTreeGroup::While {
                cond : CfrTree::bb(bb),
                then : recover_cfg_node(prims, *then, until)
            });
            _ = until.pop();
            _ = until.pop();
        },

        CfaPrim::Return => {
            tree.push(CfrTreeGroup::Block(bb));
            tree.push(CfrTreeGroup::Return);
        },

        CfaPrim::Unreachable => {
            tree.push(CfrTreeGroup::Block(bb));
            tree.push(CfrTreeGroup::Unreachable)
        }

    }
    if let Some(exit) = prim.exit() && (! until.contains(&exit)) {
        tree.append(&mut recover_cfg_node(prims, exit, until));
    }
    tree
}
