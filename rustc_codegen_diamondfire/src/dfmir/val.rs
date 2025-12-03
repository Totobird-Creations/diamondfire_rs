pub enum DfMirParamTy {
    /// Any
    Any,
    /// Variable
    Var,
    /// Value
    Val(DfMirValTy)
}

pub enum DfMirValTy {
    /// String
    Str,
    /// Styled Text
    Txt,
    /// Number
    Num,
    /// Location
    Loc,
    /// Vector
    Vec,
    /// Sound
    Snd,
    /// Particle
    Par,
    /// Potion Effect
    Pot,
    /// Item
    Item,
    /// List
    List,
    /// Dictionary
    Dict
}

pub enum DfMirVal {
    /// String
    Str(String),
    /// Styled Text
    Txt(String),
    /// Number
    Num(f64),
    /// Location
    Loc {
        x     : f64,
        y     : f64,
        z     : f64,
        pitch : f64,
        yaw   : f64
    },
    /// Vector
    Vec {
        x : f64,
        y : f64,
        z : f64
    },
    /// Sound
    Snd {
        kind  : DfMirSndKind,
        pitch : f64,
        vol   : f64
    },
    /// Particle
    Par {
        par        : String,
        count      : u16,
        spread     : (f32, f32,),
        material   : Option<String>,
        colour     : Option<DfMirCol>,
        opacity    : Option<u8>,
        size       : Option<f32>,
        motion     : Option<(f32, f32, f32,)>,
        roll       : Option<f32>,
        colour_var : Option<u8>,
        size_var   : Option<u8>,
        motion_var : Option<u8>
    },
    /// Potion Effect
    Pot {
        pot : String,
        dur : u32,
        amp : u8
    },
    /// Variable
    Var {
        name  : String,
        scope : DfMirVarScope
    },
    /// Game Value
    GameVal {
        kind   : String,
        target : DfMirGameValTarget
    }
}

pub enum DfMirSndKind {
    /// Built-in sound by name
    Sound(String),
    /// Custom sound key
    Key(String)
}

pub struct DfMirCol {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

pub enum DfMirVarScope {
    /// Line
    Line,
    /// Local
    Thread,
    /// Game
    Session,
    /// Save
    Persistent
}

pub enum DfMirGameValTarget {
    Selection,
    Default,
    Killer,
    Damager,
    Victim,
    Shooter,
    Projectile,
    LastSpawned
}
