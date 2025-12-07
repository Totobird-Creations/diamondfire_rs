use super::{
    CfaPrims,
    CfaPrim
};
use core::fmt::{ self, Display, Formatter };
use rustc_middle::mir::BasicBlock;


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
    recover_cfg_node(prims, BasicBlock::ZERO)
}


fn recover_cfg_node(
    prims : &CfaPrims,
    bb    : BasicBlock
) -> CfrTree {
    let     prim = prims.bbs.get(&bb).unwrap();
    let mut tree = CfrTree::bb(bb);
    match (prim) {

        CfaPrim::Jump { exit } => { // TODO: Handle jumping to block starts.
            tree.append(&mut recover_cfg_node(prims, *exit));
        },

        CfaPrim::If { then, exit } => {
            tree.push(CfrTreeGroup::If {
                then : recover_cfg_node(prims, *then)
            });
            tree.append(&mut recover_cfg_node(prims, *exit));
        },

        CfaPrim::IfElse { then, els, exit } => {
            tree.push(CfrTreeGroup::IfElse {
                then : recover_cfg_node(prims, *then),
                els  : recover_cfg_node(prims, *els)
            });
            tree.append(&mut recover_cfg_node(prims, *exit));
        },

        CfaPrim::Return => {
            tree.push(CfrTreeGroup::Return);
        },

        CfaPrim::Unreachable => {
            tree.push(CfrTreeGroup::Unreachable)
        }

    }
    tree
}
