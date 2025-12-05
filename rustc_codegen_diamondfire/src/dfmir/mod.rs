//! DiamondFire Mid-Level Intermediate Representation

use core::fmt::{ self, Debug, Formatter };
use std::collections::BTreeMap;

mod val;
pub use val::*;


pub struct DfMirFn {
    name        : String,
    param_count : usize,
    locals      : BTreeMap<usize, DfMirSlot>,
    temporaries : Vec<DfMirTy>,
    block_stmts : Vec<Vec<DfMirStmt>>
}

impl Debug for DfMirFn {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DfMirFn")
            .field("name", &self.name)
            .field("param_count", &self.param_count)
            .field_with("locals", |f| {
                write!(f, "{{")?;
                for (i, (local, slot,),) in self.locals.iter().enumerate() {
                    if (i != 0) { write!(f, ",")?; }
                    write!(f, " {}", local)?;
                    match (slot.refed) {
                        DfMirSlotRefState::None       => { },
                        DfMirSlotRefState::Referenced => { write!(f, " (ref)")? },
                        DfMirSlotRefState::Escaped    => { write!(f, " (esc)")? },
                    }
                    write!(f, ": {:?}", slot.ty)?;
                }
                write!(f, " }}")
            })
            .field_with("temporaries", |f| {
                write!(f, "[")?;
                for (i, ty,) in self.temporaries.iter().enumerate() {
                    if (i != 0) { write!(f, ",")?; }
                    write!(f, " {:?}", ty)?;
                }
                write!(f, " ]")
            })
            .field_with("block_stmts", |f| {
                let mut fmap = f.debug_map();
                for (i, block,) in self.block_stmts.iter().enumerate() {
                    fmap.key_with(|f| write!(f, "bb{}", i));
                    fmap.value(block);
                }
                fmap.finish()
            })
            .finish()
    }
}

impl DfMirFn {

    pub fn new(name : String) -> Self { Self {
        name,
        param_count : 0,
        locals      : BTreeMap::new(),
        temporaries : Vec::new(),
        block_stmts : Vec::new()
    } }

    pub fn push_param(&mut self, ty : DfMirTy) -> usize {
        assert_eq!(self.locals.len(), self.param_count);
        self.param_count += 1;
        self.insert_local(self.param_count, ty);
        self.param_count
    }

    pub fn insert_local(&mut self, local : usize, ty : DfMirTy) {
        _ = self.locals.try_insert(local, DfMirSlot {
            ty,
            refed : DfMirSlotRefState::None
        });
    }

    pub fn get_local_ty(&self, local : usize) -> &DfMirTy {
        if let Some(slot) = self.locals.get(&local) {
            &slot.ty
        } else { unreachable!("missing local {}", local); }
    }

    pub fn add_temporary(&mut self, ty : DfMirTy) -> usize {
        let temporary = self.temporaries.len();
        self.temporaries.push(ty);
        temporary
    }

    pub fn get_temporary_ty(&self, temporary : usize) -> &DfMirTy {
        if let Some(ty) = self.temporaries.get(temporary) {
            ty
        } else { unreachable!("missing temporary {}", temporary); }
    }

    pub fn push_block(&mut self) {
        self.block_stmts.push(Vec::new());
    }

    pub fn push_stmt(&mut self, stmt : DfMirStmt) {
        self.block_stmts.last_mut().unwrap().push(stmt);
    }

}


pub enum DfMirStmt {
    SetPlace {
        place : DfMirPlace,
        value : DfMirBasicVal
    },
    SetTemporary {
        temporary : usize,
        value     : DfMirBasicVal
    },

    AddWrapping {
        temporary : usize,
        left      : DfMirBasicVal,
        right     : DfMirBasicVal
    },
    SubWrapping {
        temporary : usize,
        left      : DfMirBasicVal,
        right     : DfMirBasicVal
    },
    GreaterThan {
        temporary : usize,
        left      : DfMirBasicVal,
        right     : DfMirBasicVal
    },

    Return
}

impl Debug for DfMirStmt {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        match (self) {
            DfMirStmt::SetPlace { place, value }               => { write!(f, "{:?} = {:?}", place, value) }
            DfMirStmt::SetTemporary { temporary, value }       => { write!(f, "temp.{} = {:?}", temporary, value) },
            DfMirStmt::AddWrapping { temporary, left, right } => { write!(f, "temp.{} = {:?} +! {:?}", temporary, left, right) },
            DfMirStmt::SubWrapping { temporary, left, right } => { write!(f, "temp.{} = {:?} -! {:?}", temporary, left, right) },
            DfMirStmt::GreaterThan { temporary, left, right }  => { write!(f, "temp.{} = {:?} > {:?}", temporary, left, right) },
            DfMirStmt::Return                                  => { write!(f, "return") }
        }
    }
}
