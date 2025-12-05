use super::{
    CfgTree,
    CfgTreeKind
};
use rustc_middle::mir::{
    Terminator,
    TerminatorKind
};
use rustc_span::Span;


#[derive(Debug)]
pub enum CfgTreePath {
    Simple,
    IfCond(Box<CfgTreePath>),
    IfThen(Box<CfgTreePath>),
    IfElse(Box<CfgTreePath>)
}

#[derive(Debug)]
pub enum CfgTerminator {
    Goto(usize),
    Return,
    Unreachable,
    Error
}


pub fn recover_cfg<'tcx>(tree : &CfgTree, terms : impl IntoIterator<Item = &'tcx Terminator<'tcx>>) -> Vec<CfgTerminator> {
    terms.into_iter().map(|term| { match (term.kind) {

        TerminatorKind::Goto { target }
        | TerminatorKind::Drop { target, .. }
        | TerminatorKind::Assert { target, .. }
        => CfgTerminator::Goto(target.index()),

        TerminatorKind::SwitchInt { .. } => todo!(),

        TerminatorKind::Return
        | TerminatorKind::TailCall { .. }
        => CfgTerminator::Return,

        TerminatorKind::Unreachable => CfgTerminator::Unreachable,

        TerminatorKind::Call { target, .. } => {
            if let Some(target) = target {
                CfgTerminator::Goto(target.index())
            } else {
                CfgTerminator::Unreachable
            }
        },

        TerminatorKind::UnwindResume
        | TerminatorKind::UnwindTerminate(_)
        | TerminatorKind::Yield { .. }
        | TerminatorKind::CoroutineDrop
        | TerminatorKind::InlineAsm { .. }
        => CfgTerminator::Error,

        TerminatorKind::FalseEdge { .. }
        | TerminatorKind::FalseUnwind { .. }
        => { unreachable!("disallowed after drop elaboration") }

    } }).collect::<Vec<_>>()
}
