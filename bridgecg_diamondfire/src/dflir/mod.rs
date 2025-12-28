use crate::VarScope;
use core::fmt::{ self, Debug, Formatter };


pub struct DfLirLine<'l> {
    pub head   : DfLirLineHead<'l>,
    pub blocks : Vec<DfLirBlock<'l>>
}
impl Debug for DfLirLine<'_> {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        let mut flist = f.debug_list();
        flist.entry(&self.head);
        for block in &self.blocks {
            flist.entry(block);
        }
        flist.finish()
    }
}


#[derive(Debug)]
pub enum DfLirLineHead<'l> {
    Function {
        name   : &'l str,
        // TODO: Icon
        // TODO: Params
        hidden : bool
    },
    Process {
        name   : &'l str,
        // TODO: Icon
        hidden : bool
    },
    PlayerEvent {
        event : &'l str
    },
    EntityEvent {
        event : &'l str
    }
}

#[derive(Debug)]
pub enum DfLirBlock<'l> {
    PlayerAction {
        action : &'l str,
        target : DfLirTarget,
        args   : Vec<DfLirValue<'l>>
        // TODO: Tags
    },
    IfPlayer {
        action : &'l str,
        not    : bool,
        target : DfLirTarget,
        args   : Vec<DfLirValue<'l>>,
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    NonPlayerAction {
        action : &'l str,
        target : DfLirTarget,
        args   : Vec<DfLirValue<'l>>
        // TODO: Tags
    },
    IfNonPlayer {
        action : &'l str,
        not    : bool,
        target : DfLirTarget,
        args   : Vec<DfLirValue<'l>>,
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    SetVar {
        action : &'l str,
        args   : Vec<DfLirValue<'l>>
        // TODO: Tags
    },
    IfVar {
        action : &'l str,
        not    : bool,
        args   : Vec<DfLirValue<'l>>,
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    GameAction {
        action : &'l str,
        args   : Vec<DfLirValue<'l>>
        // TODO: Tagss
    },
    IfGame {
        action : &'l str,
        not    : bool,
        args   : Vec<DfLirValue<'l>>,
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    Control {
        action : &'l str,
        args   : Vec<DfLirValue<'l>>
        // TODO: Tags
    },
    Repeat {
        action    : &'l str,
        subaction : Option<&'l str>,
        not       : bool,
        target    : Option<DfLirTarget>,
        args      : Vec<DfLirValue<'l>>,
        // TODO: Tags
        then      : Vec<DfLirBlock<'l>>
    },
    SelectEntity {
        action    : &'l str,
        subaction : Option<&'l str>,
        not       : bool,
        args      : Vec<DfLirValue<'l>>
        // TODO: Tags

    },
    CallFuncion {
        fn_id : u128,
        args  : Vec<DfLirValue<'l>>
    },
    StartProcess {
        name          : &'l str,
        targets       : DfLirProcessTargetMode,
        thread_locals : DfLirProcessThreadLocalMode
    }
}

impl<'l> DfLirBlock<'l> {
    pub fn copy(dst : DfLirValue<'l>, src : DfLirValue<'l>) -> Self { Self::SetVar {
        action : "=",
        args   : vec![ dst, src ]
    } }
}


#[derive(Debug)]
pub enum DfLirTarget {
    Selection,
    Default,
    Killer,
    Damager,
    Shooter,
    Victim,
    Projectile,
    LastSpawned,
    AllPlayers,
    AllMobs,
    AllNonPlayers
}

#[derive(Debug)]
pub enum DfLirProcessTargetMode {
    CopyTargets,
    CopySelection,
    None,
    EachInSelection
}

#[derive(Debug)]
pub enum DfLirProcessThreadLocalMode {
    None,
    Copy,
    Share
}


#[derive(Debug)]
pub enum DfLirValue<'l> {
    String(&'l str),
    /// This is in raw DF representation (fixed point with 3 decimals of precision).
    Number(i64),
    Var {
        scope  : VarScope,
        name   : &'l str,
        /// Indicates that this value is in a var param position and can not be optimised away.
        locked : bool
    }
}

#[doc(hidden)]
#[cfg(all(feature = "dfmir", feature = "static_intern"))]
mod __all_dfmir_staticintern {
    use super::DfLirValue;
    use crate::{
        VarScope,
        Local,
        Temporary
    };
    use static_intern::Intern as _;

    impl<'l> DfLirValue<'l> {
        pub fn var_local(temp : Local, locked : bool) -> Self {
            Self::Var { scope : VarScope::Local, name : format!("dfrs.local_{}", temp.0).intern(), locked }
        }
        pub fn var_temporary(temp : Temporary, locked : bool) -> Self {
            Self::Var { scope : VarScope::Local, name : format!("dfrs.temp_{}", temp.0).intern(), locked }
        }
    }
}
