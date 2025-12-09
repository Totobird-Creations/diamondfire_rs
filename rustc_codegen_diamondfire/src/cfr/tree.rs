use core::fmt::{ self, Display, Formatter };
use rustc_middle::mir::BasicBlock;


#[derive(Default, Debug)]
pub struct CfrTree {
    pub branches : Vec<CfrBranch>
}

#[derive(Debug)]
pub enum CfrBranch {
    Block(BasicBlock),
    Match {
        header   : BasicBlock,
        cases    : Vec<(u128, CfrTree,)>,
        fallback : CfrTree
    },
    Loop {
        header : BasicBlock,
        body   : CfrTree
    },
    Break {
        header : BasicBlock
    },
    Continue {
        header : BasicBlock
    },
    Return,
    Unreachable,
    Todo
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
            Self::Match { cases, fallback, .. } => {
                if (cases.len() == 1) {
                    write!(f, "if (...) ")?;
                    cases[0].1.fmt_indent(f, indent)?;
                    write!(f, " else ")?;
                    fallback.fmt_indent(f, indent)?;
                } else {
                    write!(f, "match (...) {{")?;
                    for (key, case,) in cases {
                        write!(f, "\n{: >indent4$}{} => ", "", key, indent4 = 4*(indent+1))?;
                        case.fmt_indent(f, indent+1)?;
                        write!(f, ",")?;
                    }
                    write!(f, "\n{: >indent4$}_ => ", "", indent4 = 4*(indent+1))?;
                    fallback.fmt_indent(f, indent+1)?;
                    write!(f, "\n{: >indent4$}}}", "", indent4 = 4*indent)?;
                }
            },
            Self::Loop { body, .. } => {
                write!(f, "loop ")?;
                body.fmt_indent(f, indent)?;
            },
            Self::Break { header } => {
                write!(f, "break 'bb{}", header.index())?;
            },
            Self::Continue { header } => {
                write!(f, "continue 'bb{}", header.index())?;
            },
            Self::Return      => { write!(f, "return")?; },
            Self::Unreachable => { write!(f, "unreachable")?; },
            Self::Todo        => { write!(f, "todo")?; }
        }
        Ok(())
    }
}
