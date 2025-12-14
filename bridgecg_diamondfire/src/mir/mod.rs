//! DiamondFire Mid-Level Intermediate Representation
//!
//! dfMIR is quite similar to rsMIR but with different statement types.

use bincode::{ Encode, Decode };

mod ty;
pub use ty::*;


#[derive(Encode, Decode, Debug)]
pub enum DfMirStmt {

    Call {
        fn_id : u128
        // TODO: Args
        // TODO: Return place
    },

    Return

}


impl DfMirStmt {

    /// Approximates the total number of dfJSON code blocks that several statements will become.
    ///
    /// Used to determine if a function is applicable for inlining.
    pub fn dfjson_cost_many(stmts : impl IntoIterator<Item = DfMirStmt>) -> usize {
        stmts.into_iter().map(|stmt| stmt.dfjson_cost()).sum()
    }

    /// Approximates the number of dfJSON code blocks this statement will become.
    pub fn dfjson_cost(&self) -> usize {
        match (self) {
            Self::Call { .. } => 1,
            Self::Return      => 1,
        }
    }

}
