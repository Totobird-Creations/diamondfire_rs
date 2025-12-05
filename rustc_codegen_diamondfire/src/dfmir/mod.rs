//! DiamondFire Mid-Level Intermediate Representation


mod val;
pub use val::*;

pub struct DfMirFn {
    pub name   : String,
    pub params : Vec<DfMirParamTy>,
    pub stmts  : Vec<DfMirStmt>
}

pub enum DfMirStmt {
    SetLineVar {
        name  : String,
        value : DfMirVal
    }
}
