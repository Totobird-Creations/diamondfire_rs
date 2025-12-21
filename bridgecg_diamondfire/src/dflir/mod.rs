use core::fmt::{ self, Debug, Formatter };


pub struct DfLirLine {
    pub head   : DfLirLineHead,
    pub blocks : Vec<DfLirBlock>
}
impl Debug for DfLirLine {
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
pub enum DfLirLineHead {
    Function {
        name   : String,
        // TODO: Icon
        // TODO: Params
        hidden : bool
    },
    Process {
        name   : String,
        // TODO: Icon
        hidden : bool
    },
    PlayerEvent {
        event : String
    },
    EntityEvent {
        event : String
    }
}

#[derive(Debug)]
pub enum DfLirBlock {
    PlayerAction {
        action : String,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
    },
    IfPlayer {
        action : String,
        not    : bool,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock>,
        els    : Vec<DfLirBlock>
    },
    NonPlayerAction {
        action : String,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
    },
    IfNonPlayer {
        action : String,
        not    : bool,
        target : DfLirTarget,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock>,
        els    : Vec<DfLirBlock>
    },
    SetVar {
        action : String,
        // TODO: Params
        // TODO: Tags
    },
    IfVar {
        action : String,
        not    : bool,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock>,
        els    : Vec<DfLirBlock>
    },
    GameAction {
        action : String,
        // TODO: Params
        // TODO: Tagss
    },
    IfGame {
        action : String,
        not    : bool,
        // TODO: Params
        // TODO: Tags
        then   : Vec<DfLirBlock>,
        els    : Vec<DfLirBlock>
    },
    Control {
        action : String,
        // TODO: Params
        // TODO: Tags
    },
    Repeat {
        action    : String,
        subaction : Option<String>,
        not       : bool,
        target    : Option<DfLirTarget>,
        // TODO: Params
        // TODO: Tags
        then      : Vec<DfLirBlock>
    },
    SelectEntity {
        action    : String,
        subaction : Option<String>,
        not       : bool
        // TODO: Params
        // TODO: Tags

    },
    CallFuncion {
        name    : String
        // TODO: Params
    },
    StartProcess {
        name          : String,
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
    LastSpawned
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
