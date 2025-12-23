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
        // TODO: Params
        // TODO: Tags
    },
    IfPlayer {
        action : &'l str,
        not    : bool,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    NonPlayerAction {
        action : &'l str,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
    },
    IfNonPlayer {
        action : &'l str,
        not    : bool,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    SetVar {
        action : &'l str,
        // TODO: Params
        // TODO: Tags
    },
    IfVar {
        action : &'l str,
        not    : bool,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    GameAction {
        action : &'l str,
        // TODO: Params
        // TODO: Tagss
    },
    IfGame {
        action : &'l str,
        not    : bool,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock<'l>>,
        els    : Vec<DfLirBlock<'l>>
    },
    Control {
        action : &'l str,
        // TODO: Params
        // TODO: Tags
    },
    Repeat {
        action    : &'l str,
        subaction : Option<&'l str>,
        not       : bool,
        target    : Option<DfLirTarget>,
        // TODO: Params
        // TODO: Tags
        then      : Vec<DfLirBlock<'l>>
    },
    SelectEntity {
        action    : &'l str,
        subaction : Option<&'l str>,
        not       : bool
        // TODO: Params
        // TODO: Tags

    },
    CallFuncion {
        fn_id : u128
        // TODO: Params
    },
    StartProcess {
        name          : &'l str,
        targets       : DfLirProcessTargetMode,
        thread_locals : DfLirProcessThreadLocalMode
    }
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
