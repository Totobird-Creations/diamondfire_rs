//! DiamondFire Mid-Level Intermediate Representation


mod val;
pub use val::*;

pub struct DfMirFn {
    name   : String,
    params : Vec<DfMirParamTy>,
    stmts  : Vec<DfMirStmt>
}

pub enum DfMirStmt {
    SetLineVar {
        name  : String,
        value : DfMirVal
    }
}
