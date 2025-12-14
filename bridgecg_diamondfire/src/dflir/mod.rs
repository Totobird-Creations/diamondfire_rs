pub struct DfLirLine {
    pub head   : DfLirLineHead,
    pub blocks : Vec<DfLirBlock>
}

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

pub enum DfLirProcessTargetMode {
    CopyTargets,
    CopySelection,
    None,
    EachInSelection
}

pub enum DfLirProcessThreadLocalMode {
    None,
    Copy,
    Share
}
